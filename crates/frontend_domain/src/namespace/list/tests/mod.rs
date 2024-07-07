use super::*;
use new_types::Pagination;

#[tokio::test]
async fn empty() {
    struct StubRepository;

    impl Repository for StubRepository {
        //
    }

    impl find_namespaces::Port for StubRepository {
        async fn find_namespaces(
            &self,
            input: find_namespaces::Input,
        ) -> Result<find_namespaces::Output, find_namespaces::Error> {
            Ok(vec![])
        }
    }

    let repository = StubRepository;
    let service = Service::new(repository);
    let input = Input {
        pagination: Pagination::from(10),
    };
    let output = service.list(input).await;

    assert!(matches!(output.as_deref(), Ok(&[])));
}
