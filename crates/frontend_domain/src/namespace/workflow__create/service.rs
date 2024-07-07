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
    async fn workflow__create(&self, input: super::Input) -> Result<super::Output, super::Error> {
        match self
            .find_namespace_by_id(super::find_namespace_by_id::Input {
                namespace_id: input.namespace_id,
            })
            .await?
        {
            None => Err(super::Error::NamespaceNotFound),
            Some(_) => match self
                .find_workflow_by_name(super::find_workflow_by_name::Input {
                    name: input.name.clone(),
                })
                .await?
            {
                Some(_) => Err(super::Error::WorkflowAlreadyExists),
                None => self
                    .create_workflow(super::create_workflow::Input {
                        namespace_id: input.namespace_id,
                        name: input.name,
                        cron: input.cron,
                        input: input.input,
                    })
                    .await
                    .map(|output| super::Output {
                        workflow_id: output.workflow_id,
                    }),
            },
        }
    }
}

impl<R> Service<R>
where
    R: super::Repository,
{
    async fn find_namespace_by_id(
        &self,
        input: super::find_namespace_by_id::Input,
    ) -> Result<super::find_namespace_by_id::Output, super::Error> {
        self.repository
            .find_namespace_by_id(input)
            .await
            .map_err(super::Error::FindNamespaceById)
    }

    async fn find_workflow_by_name(
        &self,
        input: super::find_workflow_by_name::Input,
    ) -> Result<super::find_workflow_by_name::Output, super::Error> {
        self.repository
            .find_workflow_by_name(input)
            .await
            .map_err(super::Error::FindWorkflowByName)
    }

    async fn create_workflow(
        &self,
        input: super::create_workflow::Input,
    ) -> Result<super::create_workflow::Output, super::Error> {
        self.repository
            .create_workflow(input)
            .await
            .map_err(super::Error::CreateWorkflow)
    }
}
