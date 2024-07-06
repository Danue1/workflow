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
    async fn create(&self, input: super::Input) -> Result<super::Output, super::Error> {
        match self
            .find_namespace_by_name(super::find_namespace_by_name::Input {
                name: input.name.clone(),
            })
            .await?
        {
            Some(_) => Err(super::Error::NamespaceAlreadyExists),
            None => self
                .create_namespace(super::create_namespace::Input { name: input.name })
                .await
                .map(|output| super::Output {
                    namespace_id: output.namespace_id,
                }),
        }
    }
}

impl<R> Service<R>
where
    R: super::Repository,
{
    async fn find_namespace_by_name(
        &self,
        input: super::find_namespace_by_name::Input,
    ) -> Result<super::find_namespace_by_name::Output, super::Error> {
        self.repository
            .find_namespace_by_name(input)
            .await
            .map_err(super::Error::FindNamespaceByName)
    }

    async fn create_namespace(
        &self,
        input: super::create_namespace::Input,
    ) -> Result<super::create_namespace::Output, super::Error> {
        self.repository
            .create_namespace(input)
            .await
            .map_err(super::Error::CreateNamespace)
    }
}
