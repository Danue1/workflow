use super::*;
use uuid::Uuid;

const NAMESPACE_ID: Uuid = Uuid::nil();

#[tokio::test]
async fn success() {
    struct StubRepository;

    impl Repository for StubRepository {
        //
    }

    impl remove_namespace_by_id::Port for StubRepository {
        async fn remove_namespace_by_id(
            &self,
            input: remove_namespace_by_id::Input,
        ) -> Result<remove_namespace_by_id::Output, remove_namespace_by_id::Error> {
            Ok(())
        }
    }

    let repository = StubRepository;
    let service = Service::new(repository);
    let input = Input {
        namespace_id: NAMESPACE_ID,
    };
    let output = service.remove(input).await;

    assert!(matches!(output, Ok(())));
}
