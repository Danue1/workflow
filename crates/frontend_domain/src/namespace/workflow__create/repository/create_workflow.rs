use new_types::Cron;
use uuid::Uuid;

pub trait Port: Send + Sync {
    fn create_workflow(
        &self,
        input: Input,
    ) -> impl std::future::Future<Output = Result<Output, Error>> + Send {
        async {
            std::todo!();
        }
    }
}

pub struct Input {
    pub namespace_id: Uuid,
    pub name: String,
    pub cron: Option<Cron>,
    pub input: String,
}

pub struct Output {
    pub workflow_id: Uuid,
}

#[derive(Debug)]
pub enum Error {
    Transaction(anyhow::Error), // sqlx::Error
    Commit(anyhow::Error),      // sqlx::Error
}
