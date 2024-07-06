use new_types::Pagination;
use uuid::Uuid;

pub trait Port: Send + Sync {
    fn find_namespaces(
        &self,
        input: Input,
    ) -> impl std::future::Future<Output = Result<Output, Error>> + Send {
        async {
            std::todo!();
        }
    }
}

pub struct Input {
    pub pagination: Pagination,
}

pub type Output = Vec<Namespace>;

pub struct Namespace {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub enum Error {
    Connection(anyhow::Error), // sqlx::Error
}
