use new_types::{Cron, Pagination};
use uuid::Uuid;

pub trait UseCase {
    fn workflow__list(
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
    pub pagination: Pagination,
}

pub type Output = Vec<Workflow>;

pub struct Workflow {
    pub workflow_id: Uuid,
    pub namespace_id: Uuid,
    pub name: String,
    pub cron: Option<Cron>,
    pub input: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub enum Error {
    FindWorkflows(super::find_workflows::Error),
}
