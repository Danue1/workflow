pub struct Service<R>
where
    R: super::Repository,
{
    repository: R,
}

impl<R> Service<R>
where
    R: super::Repository,
{
    #[inline]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> super::UseCase for Service<R>
where
    R: super::Repository,
{
    async fn remove(&self, input: super::Input) -> Result<super::Output, super::Error> {
        self.remove_namespace_by_id(super::remove_namespace_by_id::Input {
            namespace_id: input.namespace_id,
        })
        .await?;

        Ok(())
    }
}

impl<R> Service<R>
where
    R: super::Repository,
{
    async fn remove_namespace_by_id(
        &self,
        input: super::remove_namespace_by_id::Input,
    ) -> Result<super::remove_namespace_by_id::Output, super::Error> {
        self.repository
            .remove_namespace_by_id(input)
            .await
            .map_err(super::Error::RemoveNamespaceById)
    }
}
