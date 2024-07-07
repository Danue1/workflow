use super::*;
use new_types::Pagination;
use uuid::Uuid;

const NAMESPACE_ID: Uuid = Uuid::nil();

#[tokio::test]
async fn empty() {
    struct StubRepository;

    impl Repository for StubRepository {
        //
    }

    impl find_workflows::Port for StubRepository {
        async fn find_workflows(
            &self,
            input: find_workflows::Input,
        ) -> Result<find_workflows::Output, find_workflows::Error> {
            Ok(vec![])
        }
    }

    let repository = StubRepository;
    let service = Service::new(repository);
    let input = Input {
        namespace_id: NAMESPACE_ID,
        pagination: Pagination::from(10),
    };
    let output = service.workflow__list(input).await;

    assert!(matches!(output.as_deref(), Ok(&[])));
}
