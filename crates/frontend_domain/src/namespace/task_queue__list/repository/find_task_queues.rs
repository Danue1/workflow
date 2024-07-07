use domain::TaskQueueType;
use new_types::Pagination;
use uuid::Uuid;

pub trait Port: Send + Sync {
    fn find_task_queues(
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

pub type Output = Vec<TaskQueue>;

pub struct TaskQueue {
    pub task_queue_id: Uuid,
    pub name: String,
    pub r#type: TaskQueueType,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub enum Error {
    Connection(anyhow::Error), // sqlx::Error
}
