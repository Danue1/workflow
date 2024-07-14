use crate::{Error, ErrorKind, ParseResult};
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{cut, map, map_res, value};
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinuteRule {
    kinds: Vec<MinuteKind>,
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
pub struct MinuteKind {
    range: (Minute, Minute),
    periodic: MinutePeriodic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Minute(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MinutePeriodic(u8);

#[derive(Debug, PartialEq)]
pub enum MinuteError {
    ParseInt(std::num::ParseIntError),
    GreaterThanFiftyNine,
    PeriodicLessThanOne,
    PeriodicGreaterThanFiftyNine,
}

impl MinuteRule {
    #[inline]
    pub const fn new(kinds: Vec<MinuteKind>) -> Self {
        Self { kinds }
    }

    pub fn all() -> Self {
        let kinds = vec![MinuteKind {
            range: (Minute::MIN, Minute::MAX),
            periodic: MinutePeriodic::MIN,
        }];

        Self { kinds }
    }

    pub fn first() -> Self {
        let kinds = vec![MinuteKind {
            range: (Minute::MIN, Minute::MIN),
            periodic: MinutePeriodic::MIN,
        }];

        Self { kinds }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map(separated_list1(char(','), MinuteKind::parse), Self::new)(source)
    }
}

impl MinuteKind {
    pub const fn new(range: (Minute, Minute), periodic: MinutePeriodic) -> Self {
        Self { range, periodic }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map(
            tuple((Self::parse_range, Self::parse_periodic)),
            |(range, periodic)| Self::new(range, periodic),
        )(source)
    }

    pub fn parse_range(source: &str) -> ParseResult<(Minute, Minute)> {
        alt((
            value((Minute::MIN, Minute::MAX), char('*')),
            map(
                tuple((Minute::parse, char('-'), Minute::parse)),
                |(start, _, end)| (start, end),
            ),
        ))(source)
    }

    pub fn parse_periodic(source: &str) -> ParseResult<MinutePeriodic> {
        alt((preceded(char('/'), cut(MinutePeriodic::parse)), |source| {
            Ok((source, MinutePeriodic::MIN))
        }))(source)
    }
}

impl Minute {
    pub const MIN: Self = Self(0);
    pub const MAX: Self = Self(59);

    pub const fn new(value: u8) -> Result<Self, MinuteError> {
        if value > Minute::MAX.0 {
            Err(MinuteError::GreaterThanFiftyNine)
        } else {
            Ok(Self(value))
        }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map_res(digit1, |source: &str| {
            source
                .parse()
                .map_err(MinuteError::ParseInt)
                .and_then(Self::new)
        })(source)
    }
}

impl MinutePeriodic {
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(59);

    pub const fn new(value: u8) -> Result<Self, MinuteError> {
        if value == 0 {
            Err(MinuteError::PeriodicLessThanOne)
        } else if value > 59 {
            Err(MinuteError::PeriodicGreaterThanFiftyNine)
        } else {
            Ok(Self(value))
        }
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        map_res(digit1, |source: &str| {
            source
                .parse()
                .map_err(MinuteError::ParseInt)
                .and_then(Self::new)
        })(source)
    }
}

impl From<MinuteError> for Error {
    #[inline]
    fn from(error: MinuteError) -> Self {
        Self::new(vec![ErrorKind::Minute(error)])
    }
}

impl nom::error::FromExternalError<&str, MinuteError> for Error {
    #[inline]
    fn from_external_error(input: &str, kind: nom::error::ErrorKind, error: MinuteError) -> Self {
        Self::new(vec![
            ErrorKind::Minute(error),
            ErrorKind::Nom(input.to_string(), kind),
        ])
    }
}
