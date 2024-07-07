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
    async fn task_queue__create(&self, input: super::Input) -> Result<super::Output, super::Error> {
        match self
            .find_namespace_by_id(super::find_namespace_by_id::Input {
                namespace_id: input.namespace_id,
            })
            .await?
        {
            None => Err(super::Error::NamespaceNotFound),
            Some(_) => match self
                .find_task_queue_by_name(super::find_task_queue_by_name::Input {
                    name: input.name.clone(),
                })
                .await?
            {
                Some(_) => Err(super::Error::TaskQueueAlreadyExists),
                None => self
                    .create_task_queue(super::create_task_queue::Input {
                        namespace_id: input.namespace_id,
                        name: input.name,
                        r#type: input.r#type,
                    })
                    .await
                    .map(|output| super::Output {
                        task_queue_id: output.task_queue_id,
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

    async fn find_task_queue_by_name(
        &self,
        input: super::find_task_queue_by_name::Input,
    ) -> Result<super::find_task_queue_by_name::Output, super::Error> {
        self.repository
            .find_task_queue_by_name(input)
            .await
            .map_err(super::Error::FindTaskQueueByName)
    }

    async fn create_task_queue(
        &self,
        input: super::create_task_queue::Input,
    ) -> Result<super::create_task_queue::Output, super::Error> {
        self.repository
            .create_task_queue(input)
            .await
            .map_err(super::Error::CreateTaskQueue)
    }
}
