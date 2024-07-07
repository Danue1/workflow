use uuid::Uuid;

pub trait Port: Send + Sync {
    fn find_namespace_by_id(
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

pub type Output = Option<Namespace>;

pub struct Namespace {
    pub namespace_id: Uuid,
}

#[derive(Debug)]
pub enum Error {
    Connection(anyhow::Error), // sqlx::Error
}
