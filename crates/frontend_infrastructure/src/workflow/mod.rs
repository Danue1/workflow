pub mod list;

#[derive(Clone)]
pub struct WorkflowService {
    pool: postgresql::ConnectionPool,
}

impl WorkflowService {
    #[inline]
    pub fn new(pool: &postgresql::ConnectionPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn list(&self, input: list::Input) -> Result<list::Output, list::Error> {
        let pool = self.pool.clone();

        list::execute(input, pool).await
    }
}
