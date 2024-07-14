use crate::{Error, ErrorKind, ParseResult};
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{cut, map, map_res, value};
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecondRule {
    kinds: Vec<SecondKind>,
}

/// | Expression | Evaluation | Kind |
/// | -: | -: | -: |
/// | * | */1 | range: (0, 59), periodic: 1 |
/// | */2 | */2 | range: (0, 59), periodic: 2 |
/// | 1 | 1-1/1 | range: (1, 1), periodic: 1 |
/// | 1/2 | 1-59/2 | range: (1, 59), periodic: 2 |
/// | 1-3 | 1-3/1 | range: (1, 3), periodic: 1 |
/// | 1-3/2 | 1-3/2 | range: (1, 3), periodic: 2 |
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecondKind {
    range: (Second, Second),
    periodic: SecondPeriodic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Second(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecondPeriodic(u8);

#[derive(Debug, PartialEq)]
pub enum SecondError {
    ParseInt(std::num::ParseIntError),
    GreaterThanFiftyNine,
    PeriodicLessThanOne,
    PeriodicGreaterThanFiftyNine,
}

impl SecondRule {
    #[inline]
    pub const fn new(kinds: Vec<SecondKind>) -> Self {
        Self { kinds }
    }

    pub fn all() -> Self {
        let kinds = vec![SecondKind {
            range: (Second::MIN, Second::MAX),
            periodic: SecondPeriodic::MIN,
        }];

        Self { kinds }
    }

    pub fn first() -> Self {
        let kinds = vec![SecondKind {
            range: (Second::MIN, Second::MIN),
            periodic: SecondPeriodic::MIN,
        }];

        Self { kinds }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map(separated_list1(char(','), SecondKind::parse), Self::new)(source)
    }
}

impl SecondKind {
    pub const fn new(range: (Second, Second), periodic: SecondPeriodic) -> Self {
        Self { range, periodic }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map(
            tuple((Self::parse_range, Self::parse_periodic)),
            |(range, periodic)| Self::new(range, periodic),
        )(source)
    }

    pub fn parse_range(source: &str) -> ParseResult<(Second, Second)> {
        alt((
            value((Second::MIN, Second::MAX), char('*')),
            map(
                tuple((Second::parse, char('-'), Second::parse)),
                |(start, _, end)| (start, end),
            ),
        ))(source)
    }

    pub fn parse_periodic(source: &str) -> ParseResult<SecondPeriodic> {
        alt((preceded(char('/'), cut(SecondPeriodic::parse)), |source| {
            Ok((source, SecondPeriodic::MIN))
        }))(source)
    }
}

impl Second {
    pub const MIN: Self = Self(0);
    pub const MAX: Self = Self(59);

    pub const fn new(value: u8) -> Result<Self, SecondError> {
        if value > Second::MAX.0 {
            Err(SecondError::GreaterThanFiftyNine)
        } else {
            Ok(Self(value))
        }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map_res(digit1, |source: &str| {
            source
                .parse()
                .map_err(SecondError::ParseInt)
                .and_then(Self::new)
        })(source)
    }
}

impl SecondPeriodic {
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(59);

    pub const fn new(value: u8) -> Result<Self, SecondError> {
        if value == 0 {
            Err(SecondError::PeriodicLessThanOne)
        } else if value > 59 {
            Err(SecondError::PeriodicGreaterThanFiftyNine)
        } else {
            Ok(Self(value))
        }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map_res(digit1, |source: &str| {
            source
                .parse()
                .map_err(SecondError::ParseInt)
                .and_then(Self::new)
        })(source)
    }
}

impl From<SecondError> for Error {
    #[inline]
    fn from(error: SecondError) -> Self {
        Self::new(vec![ErrorKind::Second(error)])
    }
}

impl nom::error::FromExternalError<&str, SecondError> for Error {
    #[inline]
    fn from_external_error(input: &str, kind: nom::error::ErrorKind, error: SecondError) -> Self {
        Self::new(vec![
            ErrorKind::Second(error),
            ErrorKind::Nom(input.to_string(), kind),
        ])
    }
}
