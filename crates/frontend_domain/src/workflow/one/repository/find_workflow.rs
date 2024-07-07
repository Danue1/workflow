use new_types::Cron;
use uuid::Uuid;

pub trait Port: Send + Sync {
    fn find_workflow(
        &self,
        input: Input,
    ) -> impl std::future::Future<Output = Result<Output, Error>> + Send {
        async {
            std::todo!();
        }
    }
}

pub struct Input {
    pub workflow_id: Uuid,
}

pub type Output = Option<Workflow>;

pub struct Workflow {
    pub namespace_id: Uuid,
    pub workflow_id: Uuid,
    pub name: String,
    pub cron: Option<Cron>,
    pub input: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub enum Error {
    Connection(anyhow::Error), // sqlx::Error
}
