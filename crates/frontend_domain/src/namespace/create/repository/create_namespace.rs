use uuid::Uuid;

pub trait Port: Send + Sync {
    fn create_namespace(
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

pub struct Output {
    pub namespace_id: Uuid,
}

#[derive(Debug)]
pub enum Error {
    Transaction(anyhow::Error), // sqlx::Error
    Commit(anyhow::Error),      // sqlx::Error
}
