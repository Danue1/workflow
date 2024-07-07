use domain::TaskQueueType;
use uuid::Uuid;

pub trait Port: Send + Sync {
    fn create_task_queue(
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
    pub r#type: TaskQueueType,
}

pub struct Output {
    pub task_queue_id: Uuid,
}

#[derive(Debug)]
pub enum Error {
    Transaction(anyhow::Error), // sqlx::Error
    Commit(anyhow::Error),      // sqlx::Error
}
