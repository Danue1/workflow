#[derive(Debug, PartialEq)]
pub struct Error {
    inner: Vec<ErrorKind>,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Nom(String, nom::error::ErrorKind),
    Invalid(String, char),
    Second(super::second::SecondError),
    Minute(super::minute::MinuteError),
    Hour(super::hour::HourError),
    Date(super::date::DateError),
    Week(super::week::WeekError),
    Year(super::year::YearError),
    Month(super::month::MonthError),
}

impl Error {
    pub fn new(inner: Vec<ErrorKind>) -> Self {
        Self { inner }
    }
}

impl nom::error::ParseError<&str> for Error {
    #[inline]
    fn from_error_kind(input: &str, kind: nom::error::ErrorKind) -> Self {
        Error {
            inner: vec![ErrorKind::Nom(input.to_string(), kind)],
        }
    }

    #[inline]
    fn append(input: &str, kind: nom::error::ErrorKind, mut other: Self) -> Self {
        other.inner.push(ErrorKind::Nom(input.to_string(), kind));
        other
    }

    #[inline]
    fn from_char(input: &str, c: char) -> Self {
        Error {
            inner: vec![ErrorKind::Invalid(input.to_string(), c)],
        }
    }

    #[inline]
    fn or(self, other: Self) -> Self {
        other
    }
}

impl nom::error::FromExternalError<&str, Error> for Error {
    #[inline]
    fn from_external_error(_input: &str, _kind: nom::error::ErrorKind, error: Error) -> Self {
        error
    }
}
