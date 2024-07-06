use new_types::Pagination;

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
    async fn list(&self, input: super::Input) -> Result<super::Output, super::Error> {
        let namespaces = self
            .find_namespaces(super::find_namespaces::Input {
                pagination: input.pagination,
            })
            .await?;
        let namespaces = namespaces
            .into_iter()
            .map(|namespace| super::Namespace {
                id: namespace.id,
                name: namespace.name,
                created_at: namespace.created_at,
            })
            .collect();

        Ok(namespaces)
    }
}

impl<R> Service<R>
where
    R: super::Repository,
{
    async fn find_namespaces(
        &self,
        input: super::find_namespaces::Input,
    ) -> Result<super::find_namespaces::Output, super::Error> {
        self.repository
            .find_namespaces(input)
            .await
            .map_err(super::Error::FindNamespaces)
    }
}
