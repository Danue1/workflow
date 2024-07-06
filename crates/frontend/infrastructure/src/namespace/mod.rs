pub mod create;
pub mod list;
pub mod remove;

#[derive(Clone)]
pub struct NamespaceService {
    pool: postgresql::ConnectionPool,
}

impl NamespaceService {
    #[inline]
    pub fn new(pool: &postgresql::ConnectionPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create(&self, input: create::Input) -> Result<create::Output, create::Error> {
        let pool = self.pool.clone();

        create::execute(input, pool).await
    }

    pub async fn remove(&self, input: remove::Input) -> Result<remove::Output, remove::Error> {
        let pool = self.pool.clone();

        remove::execute(input, pool).await
    }

    pub async fn list(&self, input: list::Input) -> Result<list::Output, list::Error> {
        let pool = self.pool.clone();

        list::execute(input, pool).await
    }
}
