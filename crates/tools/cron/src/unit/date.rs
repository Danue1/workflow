use crate::{Error, ErrorKind, ParseResult};
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{cut, map, map_res, value};
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateRule {
    kinds: Vec<DateKind>,
}

/// | Expression | Evaluation | Kind |
/// | -: | -: | -: |
/// | * | 1-31/1 | range: (1, 31), periodic: 1 |
/// | */2 | 1-31/2 | range: (1, 31), periodic: 2 |
/// | 3 | 3-3/1 | range: (3, 3), periodic: 1 |
/// | 3/2 | 3-7/2 | range: (3, 7), periodic: 2 |
/// | 1-3 | 1-3/1 | range: (1, 3), periodic: 1 |
/// | 1-3/2 | 1-3/2 | range: (1, 3), periodic: 2 |
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateKind {
    range: (Date, Date),
    periodic: DatePeriodic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DatePeriodic(u8);

#[derive(Debug, PartialEq)]
pub enum DateError {
    ParseInt(std::num::ParseIntError),
    DateLessThanOne,
    DateGreaterThanThirtyOne,
    PeriodicLessThanOne,
    PeriodicGreaterThanThirtyOne,
}

impl DateRule {
    #[inline]
    pub const fn new(kinds: Vec<DateKind>) -> Self {
        Self { kinds }
    }

    pub fn all() -> Self {
        let kinds = vec![DateKind {
            range: (Date::MIN, Date::MAX),
            periodic: DatePeriodic::MIN,
        }];

        Self { kinds }
    }

    pub fn first() -> Self {
        let kinds = vec![DateKind {
            range: (Date::MIN, Date::MIN),
            periodic: DatePeriodic::MIN,
        }];

        Self { kinds }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map(separated_list1(char(','), DateKind::parse), Self::new)(source)
    }
}

impl DateKind {
    pub const fn new(range: (Date, Date), periodic: DatePeriodic) -> Self {
        Self { range, periodic }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map(
            tuple((Self::parse_range, Self::parse_periodic)),
            |(range, periodic)| Self::new(range, periodic),
        )(source)
    }

    pub fn parse_range(source: &str) -> ParseResult<(Date, Date)> {
        alt((
            value((Date::MIN, Date::MAX), char('*')),
            map(
                tuple((Date::parse, char('-'), Date::parse)),
                |(start, _, end)| (start, end),
            ),
        ))(source)
    }

    pub fn parse_periodic(source: &str) -> ParseResult<DatePeriodic> {
        alt((preceded(char('/'), cut(DatePeriodic::parse)), |source| {
            Ok((source, DatePeriodic::MIN))
        }))(source)
    }
}

impl Date {
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(31);

    pub const fn new(value: u8) -> Result<Self, DateError> {
        if value < Date::MIN.0 {
            Err(DateError::DateLessThanOne)
        } else if value > Date::MAX.0 {
            Err(DateError::DateGreaterThanThirtyOne)
        } else {
            Ok(Self(value))
        }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map_res(digit1, |source: &str| {
            source
                .parse()
                .map_err(DateError::ParseInt)
                .and_then(Date::new)
        })(source)
    }
}

impl DatePeriodic {
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(30);

    pub const fn new(value: u8) -> Result<Self, DateError> {
        if value < DatePeriodic::MIN.0 {
            Err(DateError::PeriodicLessThanOne)
        } else if value > DatePeriodic::MAX.0 {
            Err(DateError::PeriodicGreaterThanThirtyOne)
        } else {
            Ok(Self(value))
        }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map_res(digit1, |source: &str| {
            source
                .parse()
                .map_err(DateError::ParseInt)
                .and_then(DatePeriodic::new)
        })(source)
    }
}

impl From<DateError> for Error {
    #[inline]
    fn from(error: DateError) -> Self {
        Self::new(vec![ErrorKind::Date(error)])
    }
}

impl nom::error::FromExternalError<&str, DateError> for Error {
    #[inline]
    fn from_external_error(input: &str, kind: nom::error::ErrorKind, error: DateError) -> Self {
        Self::new(vec![
            ErrorKind::Date(error),
            ErrorKind::Nom(input.to_string(), kind),
        ])
    }
}
