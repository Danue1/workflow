pub struct Service<R> {
    repository: R,
}

impl<R> Service<R> {
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> super::UseCase for Service<R>
where
    R: super::Repository,
{
    async fn one(&self, input: super::Input) -> Result<super::Output, super::Error> {
        let namespace = self
            .find_namespace(super::find_namespace::Input {
                namespace_id: input.namespace_id,
            })
            .await?;

        match namespace {
            Some(namespace) => Ok(super::Namespace {
                namespace_id: namespace.namespace_id,
                name: namespace.name,
                created_at: namespace.created_at,
            }),
            None => Err(super::Error::NamespaceNotFound),
        }
    }
}

impl<R> Service<R>
where
    R: super::Repository,
{
    async fn find_namespace(
        &self,
        input: super::find_namespace::Input,
    ) -> Result<super::find_namespace::Output, super::Error> {
        self.repository
            .find_namespace(input)
            .await
            .map_err(super::Error::FindNamespace)
    }
}
