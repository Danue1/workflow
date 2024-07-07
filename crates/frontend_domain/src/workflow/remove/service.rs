pub struct Service<R> {
    repository: R,
}

impl<R> Service<R> {
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
        self.remove_workflow_by_id(super::remove_workflow_by_id::Input {
            workflow_id: input.workflow_id,
        })
        .await?;

        Ok(())
    }
}

impl<R> Service<R>
where
    R: super::Repository,
{
    async fn remove_workflow_by_id(
        &self,
        input: super::remove_workflow_by_id::Input,
    ) -> Result<super::remove_workflow_by_id::Output, super::Error> {
        self.repository
            .remove_workflow_by_id(input)
            .await
            .map_err(super::Error::RemoveWorkflowById)
    }
}
