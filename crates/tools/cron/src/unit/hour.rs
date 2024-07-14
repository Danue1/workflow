use crate::{Error, ErrorKind, ParseResult};
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{cut, map, map_res, value};
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HourRule {
    kinds: Vec<HourKind>,
}

/// | Expression | Evaluation | Kind |
/// | -: | -: | -: |
/// | * | 0-23/1 | range: (0, 23), periodic: 1 |
/// | */2 | 0-23/2 | range: (0, 23), periodic: 2 |
/// | 1 | 1-1/1 | range: (1, 1), periodic: 1 |
/// | 1/2 | 1-23/2 | range: (1, 23), periodic: 2 |
/// | 1-3 | 1-3/1 | range: (1, 3), periodic: 1 |
/// | 1-3/2 | 1-3/2 | range: (1, 3), periodic: 2 |
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HourKind {
    range: (Hour, Hour),
    periodic: HourPeriodic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hour(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HourPeriodic(u8);

#[derive(Debug, PartialEq)]
pub enum HourError {
    ParseInt(std::num::ParseIntError),
    HourLessThanOne,
    HourGreaterThanTwentyThree,
    PeriodicLessThanOne,
    PeriodicGreaterThanTwentyThree,
}

impl HourRule {
    #[inline]
    pub const fn new(kinds: Vec<HourKind>) -> Self {
        Self { kinds }
    }

    pub fn all() -> Self {
        let kinds = vec![HourKind {
            range: (Hour::MIN, Hour::MAX),
            periodic: HourPeriodic::MIN,
        }];

        Self { kinds }
    }

    pub fn first() -> Self {
        let kinds = vec![HourKind {
            range: (Hour::MIN, Hour::MIN),
            periodic: HourPeriodic::MIN,
        }];

        Self { kinds }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map(separated_list1(char(','), HourKind::parse), Self::new)(source)
    }
}

impl HourKind {
    pub fn new(range: (Hour, Hour), periodic: HourPeriodic) -> Self {
        Self { range, periodic }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map(
            tuple((Self::parse_range, Self::parse_periodic)),
            |(range, periodic)| Self::new(range, periodic),
        )(source)
    }

    pub fn parse_range(source: &str) -> ParseResult<(Hour, Hour)> {
        alt((
            value((Hour::MIN, Hour::MAX), char('*')),
            map(
                tuple((Hour::parse, char('-'), Hour::parse)),
                |(start, _, end)| (start, end),
            ),
        ))(source)
    }

    pub fn parse_periodic(source: &str) -> ParseResult<HourPeriodic> {
        alt((preceded(char('/'), cut(HourPeriodic::parse)), |source| {
            Ok((source, HourPeriodic::MIN))
        }))(source)
    }
}

impl Hour {
    pub const MIN: Self = Self(0);
    pub const MAX: Self = Self(23);

    pub const fn new(value: u8) -> Result<Self, HourError> {
        if value < Hour::MIN.0 {
            Err(HourError::HourLessThanOne)
        } else if value > Hour::MAX.0 {
            Err(HourError::HourGreaterThanTwentyThree)
        } else {
            Ok(Self(value))
        }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map_res(digit1, |source: &str| {
            source
                .parse()
                .map_err(HourError::ParseInt)
                .and_then(Self::new)
        })(source)
    }
}

impl HourPeriodic {
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(23);

    pub const fn new(value: u8) -> Result<Self, HourError> {
        if value < Self::MIN.0 {
            Err(HourError::PeriodicLessThanOne)
        } else if value > Self::MAX.0 {
            Err(HourError::PeriodicGreaterThanTwentyThree)
        } else {
            Ok(Self(value))
        }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map_res(digit1, |source: &str| {
            source
                .parse()
                .map_err(HourError::ParseInt)
                .and_then(Self::new)
        })(source)
    }
}

impl From<HourError> for Error {
    #[inline]
    fn from(error: HourError) -> Self {
        Self::new(vec![ErrorKind::Hour(error)])
    }
}

impl nom::error::FromExternalError<&str, HourError> for Error {
    #[inline]
    fn from_external_error(input: &str, kind: nom::error::ErrorKind, error: HourError) -> Self {
        Self::new(vec![
            ErrorKind::Hour(error),
            ErrorKind::Nom(input.to_string(), kind),
        ])
    }
}
