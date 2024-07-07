use uuid::Uuid;

pub trait Port: Sync + Send {
    fn remove_workflow_by_id(
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
    Transaction(anyhow::Error), // sqlx::Error
    Commit(anyhow::Error),      // sqlx::Error
}
