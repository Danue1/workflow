use new_types::Pagination;
use uuid::Uuid;

pub trait UseCase {
    fn list(
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
    FindNamespaces(super::find_namespaces::Error),
}
