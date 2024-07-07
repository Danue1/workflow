use new_types::Cron;
use uuid::Uuid;

pub trait UseCase {
    fn workflow__create(
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
    FindNamespaceById(super::find_namespace_by_id::Error),
    NamespaceNotFound,
    FindWorkflowByName(super::find_workflow_by_name::Error),
    WorkflowAlreadyExists,
    CreateWorkflow(super::create_workflow::Error),
}
