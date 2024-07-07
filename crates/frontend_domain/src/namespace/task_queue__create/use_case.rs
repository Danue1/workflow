use domain::TaskQueueType;
use uuid::Uuid;

pub trait UseCase {
    fn task_queue__create(
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
    FindNamespaceById(super::find_namespace_by_id::Error),
    NamespaceNotFound,
    FindTaskQueueByName(super::find_task_queue_by_name::Error),
    TaskQueueAlreadyExists,
    CreateTaskQueue(super::create_task_queue::Error),
}
