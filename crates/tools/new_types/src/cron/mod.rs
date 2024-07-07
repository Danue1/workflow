#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cron(::cron::Schedule);

impl Cron {
    #[inline]
    pub fn source(&self) -> String {
        self.0.to_string()
    }
}

impl AsRef<::cron::Schedule> for Cron {
    #[inline]
    fn as_ref(&self) -> &::cron::Schedule {
        &self.0
    }
}

impl std::str::FromStr for Cron {
    type Err = ::cron::error::Error;

    fn from_str(s: &str) -> Result<Cron, Self::Err> {
        ::cron::Schedule::from_str(s).map(Cron)
    }
}
