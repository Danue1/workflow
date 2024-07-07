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
            .find_workflow(super::find_workflow::Input {
                workflow_id: input.workflow_id,
            })
            .await?;

        match namespace {
            Some(namespace) => Ok(super::Workflow {
                namespace_id: namespace.namespace_id,
                workflow_id: namespace.workflow_id,
                name: namespace.name,
                cron: namespace.cron,
                input: namespace.input,
                created_at: namespace.created_at,
            }),
            None => Err(super::Error::WorkflowNotFound),
        }
    }
}

impl<R> Service<R>
where
    R: super::Repository,
{
    async fn find_workflow(
        &self,
        input: super::find_workflow::Input,
    ) -> Result<super::find_workflow::Output, super::Error> {
        self.repository
            .find_workflow(input)
            .await
            .map_err(super::Error::FindWorkflow)
    }
}
