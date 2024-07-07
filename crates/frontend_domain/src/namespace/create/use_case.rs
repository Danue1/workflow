use uuid::Uuid;

pub trait UseCase {
    fn create(
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
    FindNamespaceByName(super::find_namespace_by_name::Error),
    NamespaceAlreadyExists,
    CreateNamespace(super::create_namespace::Error),
}
