use uuid::Uuid;

pub trait UseCase {
    fn remove(
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
    RemoveNamespaceById(super::remove_namespace_by_id::Error),
}
