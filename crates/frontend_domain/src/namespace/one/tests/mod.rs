use super::*;
use uuid::Uuid;

#[tokio::test]
async fn found() {
    const NAMESPACE_ID: Uuid = Uuid::nil();

    struct StubRepository;

    impl Repository for StubRepository {
        //
    }

    impl find_namespace::Port for StubRepository {
        async fn find_namespace(
            &self,
            input: find_namespace::Input,
        ) -> Result<find_namespace::Output, find_namespace::Error> {
            Ok(Some(find_namespace::Namespace {
                namespace_id: input.namespace_id,
                name: "namespace".to_owned(),
                created_at: chrono::DateTime::<chrono::Utc>::MIN_UTC,
            }))
        }
    }

    let repository = StubRepository;
    let service = Service::new(repository);
    let input = Input {
        namespace_id: NAMESPACE_ID,
    };
    let output = service.one(input).await;

    assert!(matches!(
        output,
        Ok(Namespace {
            namespace_id: NAMESPACE_ID,
            name,
            created_at,
        })
    ));
}

#[tokio::test]
async fn not_found() {
    const NAMESPACE_ID: Uuid = Uuid::nil();

    struct StubRepository;

    impl Repository for StubRepository {
        //
    }

    impl find_namespace::Port for StubRepository {
        async fn find_namespace(
            &self,
            input: find_namespace::Input,
        ) -> Result<find_namespace::Output, find_namespace::Error> {
            Ok(None)
        }
    }

    let repository = StubRepository;
    let service = Service::new(repository);
    let input = Input {
        namespace_id: NAMESPACE_ID,
    };
    let output = service.one(input).await;

    assert!(matches!(output, Err(Error::NamespaceNotFound)));
}
