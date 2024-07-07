use uuid::Uuid;

pub trait UseCase {
    fn remove(
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

pub type Output = ();

#[derive(Debug)]
pub enum Error {
    RemoveWorkflowById(super::remove_workflow_by_id::Error),
}
