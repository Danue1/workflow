use uuid::Uuid;

pub trait Port: Sync + Send {
    fn remove_namespace_by_id(
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
}

pub type Output = ();

#[derive(Debug)]
pub enum Error {
    Transaction(anyhow::Error), // sqlx::Error
    Commit(anyhow::Error),      // sqlx::Error
}
