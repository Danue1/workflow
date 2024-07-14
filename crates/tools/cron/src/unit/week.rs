use crate::{Error, ErrorKind, ParseResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::{cut, map, map_res, value};
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeekRule {
    pub(crate) kinds: Vec<WeekKind>,
}

/// | Expression | Evaluation | Kind |
/// | -: | -: | -: |
/// | * | 1-7/1 | range: (1, 7), periodic: 1 |
/// | */2 | 1-7/2 | range: (1, 7), periodic: 2 |
/// | 3 | 3-3/1 | range: (3, 3), periodic: 1 |
/// | 3/2 | 3-7/2 | range: (3, 7), periodic: 2 |
/// | 1-3 | 1-3/1 | range: (1, 3), periodic: 1 |
/// | 1-3/2 | 1-3/2 | range: (1, 3), periodic: 2 |
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeekKind {
    range: (Week, Week),
    periodic: WeekPeriodic,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Week {
    #[default]
    Sunday = 1,
    Monday = 2,
    Tuesday = 3,
    Wednesday = 4,
    Thursday = 5,
    Friday = 6,
    Saturday = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WeekPeriodic(u8);

#[derive(Debug, PartialEq)]
pub enum WeekError {
    ParseInt(std::num::ParseIntError),
    DayLessThanOne,
    DayGreaterThanSeven,
    PeriodicLessThanOne,
    PeriodicGreaterThanSeven,
}

impl WeekRule {
    #[inline]
    pub const fn new(kinds: Vec<WeekKind>) -> Self {
        Self { kinds }
    }

    pub fn all() -> Self {
        let kinds = vec![WeekKind {
            range: (Week::Sunday, Week::Saturday),
            periodic: WeekPeriodic::MIN,
        }];

        Self { kinds }
    }

    pub fn first() -> Self {
        let kinds = vec![WeekKind {
            range: (Week::Sunday, Week::Sunday),
            periodic: WeekPeriodic::MIN,
        }];

        Self { kinds }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map(separated_list1(char(','), WeekKind::parse), Self::new)(source)
    }
}

impl WeekKind {
    pub const fn new(range: (Week, Week), periodic: WeekPeriodic) -> Self {
        Self { range, periodic }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map(
            tuple((Self::parse_range, Self::parse_periodic)),
            |(range, periodic)| Self::new(range, periodic),
        )(source)
    }

    pub fn parse_range(source: &str) -> ParseResult<(Week, Week)> {
        alt((
            value((Week::MIN, Week::MAX), char('*')),
            map(
                tuple((Week::parse, tag("-"), Week::parse)),
                |(start, _, end)| (start, end),
            ),
        ))(source)
    }

    pub fn parse_periodic(source: &str) -> ParseResult<WeekPeriodic> {
        alt((preceded(char('/'), cut(WeekPeriodic::parse)), |source| {
            Ok((source, WeekPeriodic::MIN))
        }))(source)
    }
}

impl Week {
    pub const MIN: Self = Self::Sunday;
    pub const MAX: Self = Self::Saturday;

    pub fn parse(source: &str) -> ParseResult<Self> {
        alt((Self::parse_numeric, Self::parse_alpha))(source)
    }

    pub fn parse_numeric(source: &str) -> ParseResult<Self> {
        alt((
            value(Self::Sunday, tag("1")),
            value(Self::Monday, tag("2")),
            value(Self::Tuesday, tag("3")),
            value(Self::Wednesday, tag("4")),
            value(Self::Thursday, tag("5")),
            value(Self::Friday, tag("6")),
            value(Self::Saturday, tag("7")),
        ))(source)
    }

    pub fn parse_alpha(source: &str) -> ParseResult<Self> {
        alt((
            value(Self::Sunday, tag("SUN")),
            value(Self::Monday, tag("MON")),
            value(Self::Tuesday, tag("TUE")),
            value(Self::Wednesday, tag("WED")),
            value(Self::Thursday, tag("THU")),
            value(Self::Friday, tag("FRI")),
            value(Self::Saturday, tag("SAT")),
        ))(source)
    }
}

impl WeekPeriodic {
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(7);

    pub const fn new(value: u8) -> Result<Self, WeekError> {
        if value < Self::MIN.0 {
            Err(WeekError::PeriodicLessThanOne)
        } else if value > Self::MAX.0 {
            Err(WeekError::PeriodicGreaterThanSeven)
        } else {
            Ok(Self(value))
        }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map_res(digit1, |source: &str| {
            source
                .parse()
                .map_err(WeekError::ParseInt)
                .and_then(WeekPeriodic::new)
        })(source)
    }
}

impl From<WeekError> for Error {
    #[inline]
    fn from(error: WeekError) -> Self {
        Self::new(vec![ErrorKind::Week(error)])
    }
}

impl nom::error::FromExternalError<&str, WeekError> for Error {
    #[inline]
    fn from_external_error(input: &str, kind: nom::error::ErrorKind, error: WeekError) -> Self {
        Self::new(vec![
            ErrorKind::Week(error),
            ErrorKind::Nom(input.to_string(), kind),
        ])
    }
}
