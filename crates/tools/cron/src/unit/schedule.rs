use crate::{DateRule, HourRule, MinuteRule, SecondRule};
use crate::{Error, ParseResult};
use crate::{Month, MonthKind, MonthPeriodic, MonthRule};
use crate::{Week, WeekKind, WeekPeriodic, WeekRule};
use crate::{Year, YearKind, YearPeriodic, YearRule};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{all_consuming, map_res};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schedule {
    pub source: String,
    pub second: SecondRule,
    pub minute: MinuteRule,
    pub hour: HourRule,
    pub date: DateRule,
    pub week: WeekRule,
    pub month: MonthRule,
    pub year: YearRule,
}

impl Schedule {
    pub fn new_secondly(source: &str) -> Result<Self, Error> {
        Ok(Self {
            source: source.to_owned(),
            second: SecondRule::all(),
            minute: MinuteRule::all(),
            hour: HourRule::all(),
            date: DateRule::all(),
            week: WeekRule::all(),
            month: MonthRule::all(),
            year: YearRule::all(),
        })
    }

    pub fn new_minutely(source: &str) -> Result<Self, Error> {
        Ok(Self {
            source: source.to_owned(),
            second: SecondRule::first(),
            minute: MinuteRule::all(),
            hour: HourRule::all(),
            date: DateRule::all(),
            week: WeekRule::all(),
            month: MonthRule::all(),
            year: YearRule::all(),
        })
    }

    pub fn new_hourly(source: &str) -> Result<Self, Error> {
        Ok(Self {
            source: source.to_owned(),
            second: SecondRule::first(),
            minute: MinuteRule::first(),
            hour: HourRule::all(),
            date: DateRule::all(),
            week: WeekRule::all(),
            month: MonthRule::all(),
            year: YearRule::all(),
        })
    }

    pub fn new_daily(source: &str) -> Result<Self, Error> {
        Ok(Self {
            source: source.to_owned(),
            second: SecondRule::first(),
            minute: MinuteRule::first(),
            hour: HourRule::first(),
            date: DateRule::all(),
            week: WeekRule::all(),
            month: MonthRule::all(),
            year: YearRule::all(),
        })
    }

    pub fn new_weekly(source: &str, day: Week) -> Result<Self, Error> {
        Ok(Self {
            source: source.to_owned(),
            second: SecondRule::first(),
            minute: MinuteRule::first(),
            hour: HourRule::first(),
            date: DateRule::first(),
            week: WeekRule::new(vec![WeekKind::new((day, day), WeekPeriodic::new(1)?)]),
            month: MonthRule::all(),
            year: YearRule::all(),
        })
    }

    pub fn new_monthly(source: &str) -> Result<Self, Error> {
        Ok(Self {
            source: source.to_owned(),
            second: SecondRule::first(),
            minute: MinuteRule::first(),
            hour: HourRule::first(),
            date: DateRule::first(),
            week: WeekRule::first(),
            month: MonthRule::all(),
            year: YearRule::all(),
        })
    }

    pub fn new_periodic_monthly(
        source: &str,
        start: Month,
        periodic: MonthPeriodic,
    ) -> Result<Self, Error> {
        Ok(Self {
            source: source.to_owned(),
            second: SecondRule::first(),
            minute: MinuteRule::first(),
            hour: HourRule::first(),
            date: DateRule::first(),
            week: WeekRule::first(),
            month: MonthRule::new(vec![MonthKind::new((start, Month::MAX), periodic)]),
            year: YearRule::all(),
        })
    }

    pub fn new_yearly(source: &str) -> Result<Self, Error> {
        Ok(Self {
            source: source.to_owned(),
            second: SecondRule::first(),
            minute: MinuteRule::first(),
            hour: HourRule::first(),
            date: DateRule::first(),
            week: WeekRule::first(),
            month: MonthRule::first(),
            year: YearRule::all(),
        })
    }

    pub fn new_periodic_yearly(
        source: &str,
        start: u16,
        periodic: YearPeriodic,
    ) -> Result<Self, Error> {
        Ok(Self {
            source: source.to_owned(),
            second: SecondRule::first(),
            minute: MinuteRule::first(),
            hour: HourRule::first(),
            date: DateRule::first(),
            week: WeekRule::first(),
            month: MonthRule::first(),
            year: YearRule::new(vec![YearKind::new(
                (Year::MIN + start, Year::MAX),
                periodic,
            )]),
        })
    }

    pub fn parse(source: &str) -> ParseResult<Self> {
        all_consuming(alt((Self::parse_short_hand, Self::parse_long_hand)))(source)
    }

    pub fn parse_short_hand(source: &str) -> ParseResult<Self> {
        alt((
            Self::parse_secondly,
            Self::parse_minutely,
            Self::parse_hourly,
            Self::parse_daily,
            Self::parse_weekly,
            Self::parse_sunday,
            Self::parse_monday,
            Self::parse_tuesday,
            Self::parse_wednesday,
            Self::parse_thursday,
            Self::parse_friday,
            Self::parse_saturday,
            Self::parse_monthly,
            Self::parse_even_monthly,
            Self::parse_odd_monthly,
            Self::parse_yearly,
            Self::parse_annually,
            Self::parse_even_yearly,
            Self::parse_odd_yearly,
            Self::parse_leap_yearly,
        ))(source)
    }

    pub fn parse_long_hand(source: &str) -> ParseResult<Self> {
        let origin = source;
        let (source, second) = SecondRule::parse(source)?;
        let (source, _) = char(' ')(source)?;
        let (source, minute) = MinuteRule::parse(source)?;
        let (source, _) = char(' ')(source)?;
        let (source, hour) = HourRule::parse(source)?;
        let (source, _) = char(' ')(source)?;
        let (source, date) = DateRule::parse(source)?;
        let (source, _) = char(' ')(source)?;
        let (source, week) = WeekRule::parse(source)?;
        let (source, _) = char(' ')(source)?;
        let (source, month) = MonthRule::parse(source)?;
        let (source, _) = char(' ')(source)?;
        let (source, year) = YearRule::parse(source)?;

        let schedule = Self {
            source: origin.to_owned(),
            second,
            minute,
            hour,
            date,
            week,
            month,
            year,
        };

        Ok((source, schedule))
    }

    pub fn parse_secondly(source: &str) -> ParseResult<Self> {
        map_res(tag("@secondly"), |_| Self::new_secondly(source))(source)
    }

    pub fn parse_minutely(source: &str) -> ParseResult<Self> {
        map_res(tag("@minutely"), |_| Self::new_minutely(source))(source)
    }

    pub fn parse_hourly(source: &str) -> ParseResult<Self> {
        map_res(tag("@hourly"), |_| Self::new_hourly(source))(source)
    }

    pub fn parse_daily(source: &str) -> ParseResult<Self> {
        map_res(tag("@daily"), |_| Self::new_daily(source))(source)
    }

    pub fn parse_weekly(source: &str) -> ParseResult<Self> {
        map_res(tag("@weekly"), |_| Self::new_weekly(source, Week::Sunday))(source)
    }

    pub fn parse_sunday(source: &str) -> ParseResult<Self> {
        map_res(tag("@sunday"), |_| Self::new_weekly(source, Week::Sunday))(source)
    }

    pub fn parse_monday(source: &str) -> ParseResult<Self> {
        map_res(tag("@monday"), |_| Self::new_weekly(source, Week::Monday))(source)
    }

    pub fn parse_tuesday(source: &str) -> ParseResult<Self> {
        map_res(tag("@tuesday"), |_| Self::new_weekly(source, Week::Tuesday))(source)
    }

    pub fn parse_wednesday(source: &str) -> ParseResult<Self> {
        map_res(tag("@wednesday"), |_| {
            Self::new_weekly(source, Week::Wednesday)
        })(source)
    }

    pub fn parse_thursday(source: &str) -> ParseResult<Self> {
        map_res(tag("@thursday"), |_| {
            Self::new_weekly(source, Week::Thursday)
        })(source)
    }

    pub fn parse_friday(source: &str) -> ParseResult<Self> {
        map_res(tag("@friday"), |_| Self::new_weekly(source, Week::Friday))(source)
    }

    pub fn parse_saturday(source: &str) -> ParseResult<Self> {
        map_res(tag("@saturday"), |_| {
            Self::new_weekly(source, Week::Saturday)
        })(source)
    }

    pub fn parse_monthly(source: &str) -> ParseResult<Self> {
        map_res(tag("@monthly"), |_| Self::new_monthly(source))(source)
    }

    pub fn parse_even_monthly(source: &str) -> ParseResult<Self> {
        map_res(tag("@even-monthly"), |_| {
            Self::new_periodic_monthly(source, Month::February, MonthPeriodic::new(2)?)
        })(source)
    }

    pub fn parse_odd_monthly(source: &str) -> ParseResult<Self> {
        map_res(tag("@odd-monthly"), |_| {
            Self::new_periodic_monthly(source, Month::January, MonthPeriodic::new(2)?)
        })(source)
    }

    pub fn parse_yearly(source: &str) -> ParseResult<Self> {
        map_res(tag("@yearly"), |_| Self::new_yearly(source))(source)
    }

    pub fn parse_annually(source: &str) -> ParseResult<Self> {
        map_res(tag("@annually"), |_| Self::new_yearly(source))(source)
    }

    pub fn parse_even_yearly(source: &str) -> ParseResult<Self> {
        map_res(tag("@even-yearly"), |_| {
            Self::new_periodic_yearly(source, 0, YearPeriodic::new(2)?)
        })(source)
    }

    pub fn parse_odd_yearly(source: &str) -> ParseResult<Self> {
        map_res(tag("@odd-yearly"), |_| {
            Self::new_periodic_yearly(source, 1, YearPeriodic::new(2)?)
        })(source)
    }

    pub fn parse_leap_yearly(source: &str) -> ParseResult<Self> {
        map_res(tag("@leap-yearly"), |_| {
            Self::new_periodic_yearly(source, 0, YearPeriodic::new(4)?)
        })(source)
    }
}
