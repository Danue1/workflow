use crate::ErrorKind;

use super::{Error, ParseResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::{cut, map, map_res, opt, value};
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonthRule {
    pub(crate) kinds: Vec<MonthKind>,
}

/// | Expression | Evaluation | Kind |
/// | -: | -: | -: |
/// | * | */1 | range: (1, 12), periodic: 1 |
/// | */2 | */2 | range: (1, 12), periodic: 2 |
/// | 1 | 1-1/1 | range: (1, 1), periodic: 1 |
/// | 1/2 | 1-1/2 | range: (1, 1), periodic: 2 |
/// | 1-3 | 1-3/1 | range: (1, 3), periodic: 1 |
/// | 1-3/2 | 1-3/2 | range: (1, 3), periodic: 2 |
/// | JAN | 1-1/1 | range: (1, 1), periodic: 1 |
/// | JAN/2 | 1-1/2 | range: (1, 1), periodic: 2 |
/// | JAN-MAR | 1-3/1 | range: (1, 3), periodic: 1 |
/// | JAN-MAR/2 | 1-3/2 | range: (1, 3), periodic: 2 |
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonthKind {
    range: (Month, Month),
    periodic: MonthPeriodic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MonthPeriodic(u8);

#[derive(Debug, PartialEq)]
pub enum MonthError {
    ParseInt(std::num::ParseIntError),
    MonthLessThanOne,
    MonthGreaterThanTwelve,
}

impl MonthRule {
    #[inline]
    pub const fn new(kinds: Vec<MonthKind>) -> Self {
        Self { kinds }
    }

    pub fn all() -> Self {
        let kinds = vec![MonthKind {
            range: (Month::January, Month::December),
            periodic: MonthPeriodic::MIN,
        }];

        Self { kinds }
    }

    pub fn first() -> Self {
        let kinds = vec![MonthKind {
            range: (Month::January, Month::January),
            periodic: MonthPeriodic::MIN,
        }];

        Self { kinds }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map(separated_list1(char(','), MonthKind::parse), Self::new)(source)
    }
}

impl MonthKind {
    pub const fn new(range: (Month, Month), periodic: MonthPeriodic) -> Self {
        Self { range, periodic }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map(
            tuple((Self::parse_range, Self::parse_periodic)),
            |(range, periodic)| Self::new(range, periodic),
        )(source)
    }

    pub fn parse_range(source: &str) -> ParseResult<(Month, Month)> {
        alt((
            value((Month::MIN, Month::MAX), char('*')),
            map(
                tuple((Month::parse, opt(preceded(char('-'), Month::parse)))),
                |(start, end)| (start, end.unwrap_or(start)),
            ),
        ))(source)
    }

    pub fn parse_periodic(source: &str) -> ParseResult<MonthPeriodic> {
        alt((preceded(char('/'), cut(MonthPeriodic::parse)), |source| {
            Ok((source, MonthPeriodic::MIN))
        }))(source)
    }
}

impl Month {
    pub const MIN: Self = Self::January;
    pub const MAX: Self = Self::December;

    pub fn parse(source: &str) -> ParseResult<Self> {
        alt((Self::parse_numeric, Self::parse_alpha))(source)
    }

    pub fn parse_numeric(source: &str) -> ParseResult<Self> {
        alt((
            value(Self::December, tag("12")),
            value(Self::November, tag("11")),
            value(Self::October, tag("10")),
            value(Self::September, tag("9")),
            value(Self::August, tag("8")),
            value(Self::July, tag("7")),
            value(Self::June, tag("6")),
            value(Self::May, tag("5")),
            value(Self::April, tag("4")),
            value(Self::March, tag("3")),
            value(Self::February, tag("2")),
            value(Self::January, tag("1")),
        ))(source)
    }

    pub fn parse_alpha(source: &str) -> ParseResult<Self> {
        alt((
            value(Self::January, tag("JAN")),
            value(Self::February, tag("FEB")),
            value(Self::March, tag("MAR")),
            value(Self::April, tag("APR")),
            value(Self::May, tag("MAY")),
            value(Self::June, tag("JUN")),
            value(Self::July, tag("JUL")),
            value(Self::August, tag("AUG")),
            value(Self::September, tag("SEP")),
            value(Self::October, tag("OCT")),
            value(Self::November, tag("NOV")),
            value(Self::December, tag("DEC")),
        ))(source)
    }
}

impl MonthPeriodic {
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(12);

    pub const fn new(value: u8) -> Result<Self, MonthError> {
        if value == Self::MIN.0 {
            Err(MonthError::MonthLessThanOne)
        } else if value > Self::MAX.0 {
            Err(MonthError::MonthGreaterThanTwelve)
        } else {
            Ok(Self(value))
        }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map_res(digit1, |source: &str| {
            source
                .parse()
                .map_err(MonthError::ParseInt)
                .and_then(Self::new)
        })(source)
    }
}

impl From<MonthError> for Error {
    #[inline]
    fn from(error: MonthError) -> Self {
        Self::new(vec![ErrorKind::Month(error)])
    }
}

impl nom::error::FromExternalError<&str, MonthError> for Error {
    #[inline]
    fn from_external_error(input: &str, kind: nom::error::ErrorKind, error: MonthError) -> Self {
        Self::new(vec![
            ErrorKind::Month(error),
            ErrorKind::Nom(input.to_string(), kind),
        ])
    }
}
