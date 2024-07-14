use crate::ErrorKind;

use super::{Error, ParseResult};
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{cut, map, map_res, opt, value};
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YearRule {
    kinds: Vec<YearKind>,
}

/// | Expression | Evaluation | Kind |
/// | -: | -: | -: |
/// | * | 1970-2199/1 | range: (1970, 2199), periodic: 1 |
/// | */2 | 1970-2199/2 | range: (1970, 2199), periodic: 2 |
/// | 2024 | 2024-2024/1 | range: (2024, 2024), periodic: 1 |
/// | 2024/2 | 2024-2199/2 | range: (2024, 2199), periodic: 2 |
/// | 2024-2026 | 2024-2026/1 | range: (2024, 2026), periodic: 1 |
/// | 2024-2026/2 | 2024-2199/2 | range: (2024, 2199), periodic: 2 |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearKind {
    range: (Year, Year),
    periodic: YearPeriodic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Year(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct YearPeriodic(u8);

#[derive(Debug, PartialEq)]
pub enum YearError {
    ParseInt(std::num::ParseIntError),
    YearLessThan1970,
    YearGreaterThan2199,
    RangeLessThanStart,
    PeriodicLessThanOne,
}

impl YearRule {
    #[inline]
    pub const fn new(kinds: Vec<YearKind>) -> Self {
        Self { kinds }
    }

    pub fn all() -> Self {
        let kinds = vec![YearKind {
            range: (Year::MIN, Year::MAX),
            periodic: YearPeriodic::MIN,
        }];

        Self { kinds }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map(separated_list1(char(','), YearKind::parse), Self::new)(source)
    }
}

impl YearKind {
    pub const fn new(range: (Year, Year), periodic: YearPeriodic) -> Self {
        Self { range, periodic }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map(
            tuple((Self::parse_range, Self::parse_periodic)),
            |(range, periodic)| Self::new(range, periodic),
        )(source)
    }

    pub fn parse_range(source: &str) -> ParseResult<(Year, Year)> {
        alt((
            value((Year::MIN, Year::MAX), char('*')),
            map_res(
                tuple((Year::parse, opt(preceded(char('-'), Year::parse)))),
                |(start, end)| match (start, end) {
                    (start, None) => Ok((start, start)),
                    (start, Some(end)) if end >= start => Ok((start, end)),
                    _ => Err(YearError::RangeLessThanStart),
                },
            ),
        ))(source)
    }

    pub fn parse_periodic(source: &str) -> ParseResult<YearPeriodic> {
        alt((preceded(char('/'), cut(YearPeriodic::parse)), |source| {
            Ok((source, YearPeriodic::MIN))
        }))(source)
    }
}

impl Year {
    pub const MIN: Self = Self(1970);
    pub const MAX: Self = Self(2199);

    pub const fn new(value: u16) -> Result<Self, YearError> {
        if value < Year::MIN.0 {
            Err(YearError::YearLessThan1970)
        } else if value > Year::MAX.0 {
            Err(YearError::YearGreaterThan2199)
        } else {
            Ok(Self(value))
        }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map_res(digit1, |source: &str| {
            source
                .parse()
                .map_err(YearError::ParseInt)
                .and_then(Year::new)
        })(source)
    }
}

impl std::ops::Add<u16> for Year {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl YearPeriodic {
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(255);

    pub const fn new(value: u8) -> Result<Self, YearError> {
        if value == 0 {
            Err(YearError::PeriodicLessThanOne)
        } else {
            Ok(Self(value))
        }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map_res(digit1, |source: &str| {
            source
                .parse()
                .map_err(YearError::ParseInt)
                .and_then(Self::new)
        })(source)
    }
}

impl From<YearError> for Error {
    #[inline]
    fn from(error: YearError) -> Self {
        Self::new(vec![ErrorKind::Year(error)])
    }
}

impl nom::error::FromExternalError<&str, YearError> for Error {
    #[inline]
    fn from_external_error(input: &str, kind: nom::error::ErrorKind, error: YearError) -> Self {
        Self::new(vec![
            ErrorKind::Year(error),
            ErrorKind::Nom(input.to_string(), kind),
        ])
    }
}
