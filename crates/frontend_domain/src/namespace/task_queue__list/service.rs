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
    async fn task_queue__list(&self, input: super::Input) -> Result<super::Output, super::Error> {
        let task_queues = self
            .find_task_queues(super::find_task_queues::Input {
                namespace_id: input.namespace_id,
                pagination: input.pagination,
            })
            .await?;
        let task_queues = task_queues
            .into_iter()
            .map(|task_queue| super::TaskQueue {
                task_queue_id: task_queue.task_queue_id,
                name: task_queue.name,
                r#type: task_queue.r#type,
                created_at: task_queue.created_at,
            })
            .collect();

        Ok(task_queues)
    }
}

impl<R> Service<R>
where
    R: super::Repository,
{
    async fn find_task_queues(
        &self,
        input: super::find_task_queues::Input,
    ) -> Result<super::find_task_queues::Output, super::Error> {
        self.repository
            .find_task_queues(input)
            .await
            .map_err(super::Error::FindTaskQueues)
    }
}
