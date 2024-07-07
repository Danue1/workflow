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
    async fn workflow__list(&self, input: super::Input) -> Result<super::Output, super::Error> {
        let workflows = self
            .find_workflows(super::find_workflows::Input {
                namespace_id: input.namespace_id,
                pagination: input.pagination,
            })
            .await?;
        let workflows = workflows
            .into_iter()
            .map(|workflow| super::Workflow {
                workflow_id: workflow.workflow_id,
                namespace_id: workflow.namespace_id,
                name: workflow.name,
                cron: workflow.cron,
                input: workflow.input,
                created_at: workflow.created_at,
            })
            .collect();

        Ok(workflows)
    }
}

impl<R> Service<R>
where
    R: super::Repository,
{
    async fn find_workflows(
        &self,
        input: super::find_workflows::Input,
    ) -> Result<super::find_workflows::Output, super::Error> {
        self.repository
            .find_workflows(input)
            .await
            .map_err(super::Error::FindWorkflows)
    }
}
