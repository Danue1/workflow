use uuid::Uuid;

pub trait Port: Send + Sync {
    fn find_workflow_by_name(
        &self,
        input: Input,
    ) -> impl std::future::Future<Output = Result<Output, Error>> + Send {
        async {
            std::todo!();
        }
    }
}

pub struct Input {
    pub name: String,
}

pub type Output = Option<Workflow>;

pub struct Workflow {
    pub workflow_id: Uuid,
}

#[derive(Debug)]
pub enum Error {
    Connection(anyhow::Error), // sqlx::Error
}
