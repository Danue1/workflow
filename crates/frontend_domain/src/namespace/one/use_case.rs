use uuid::Uuid;

pub trait UseCase {
    fn one(&self, input: Input) -> impl std::future::Future<Output = Result<Output, Error>> + Send {
        async {
            std::todo!();
        }
    }
}

pub struct Input {
    pub namespace_id: Uuid,
}

pub type Output = Namespace;

pub struct Namespace {
    pub namespace_id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub enum Error {
    FindNamespace(super::find_namespace::Error),
    NamespaceNotFound,
}
