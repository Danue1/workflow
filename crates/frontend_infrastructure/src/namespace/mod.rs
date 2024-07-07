pub mod create;
pub mod list;
pub mod one;
pub mod remove;
pub mod task_queue__create;
pub mod workflow__create;

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

    pub async fn one(&self, input: one::Input) -> Result<one::Output, one::Error> {
        let pool = self.pool.clone();

        one::execute(input, pool).await
    }

    pub async fn task_queue__create(
        &self,
        input: task_queue__create::Input,
    ) -> Result<task_queue__create::Output, task_queue__create::Error> {
        let pool = self.pool.clone();

        task_queue__create::execute(input, pool).await
    }

    pub async fn workflow__create(
        &self,
        input: workflow__create::Input,
    ) -> Result<workflow__create::Output, workflow__create::Error> {
        let pool = self.pool.clone();

        workflow__create::execute(input, pool).await
    }
}
