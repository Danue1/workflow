use super::*;
use uuid::Uuid;

const NAMESPACE_ID: Uuid = Uuid::nil();

#[tokio::test]
async fn success() {
    struct StubRepository;

    impl Repository for StubRepository {
        //
    }

    impl find_namespace_by_name::Port for StubRepository {
        async fn find_namespace_by_name(
            &self,
            input: find_namespace_by_name::Input,
        ) -> Result<find_namespace_by_name::Output, find_namespace_by_name::Error> {
            Ok(None)
        }
    }

    impl create_namespace::Port for StubRepository {
        async fn create_namespace(
            &self,
            input: create_namespace::Input,
        ) -> Result<create_namespace::Output, create_namespace::Error> {
            Ok(create_namespace::Output {
                namespace_id: NAMESPACE_ID,
            })
        }
    }

    let repository = StubRepository;
    let service = Service::new(repository);
    let input = Input {
        name: "name".to_string(),
    };
    let output = service.create(input).await;

    assert!(matches!(
        output,
        Ok(Output {
            namespace_id: NAMESPACE_ID
        })
    ));
}

#[tokio::test]
async fn already_exists() {
    struct StubRepository;

    impl Repository for StubRepository {
        //
    }

    impl find_namespace_by_name::Port for StubRepository {
        async fn find_namespace_by_name(
            &self,
            input: find_namespace_by_name::Input,
        ) -> Result<find_namespace_by_name::Output, find_namespace_by_name::Error> {
            Ok(Some(find_namespace_by_name::Namespace {
                namespace_id: NAMESPACE_ID,
            }))
        }
    }

    impl create_namespace::Port for StubRepository {
        //
    }

    let repository = StubRepository;
    let service = Service::new(repository);
    let input = Input {
        name: "name".to_string(),
    };
    let output = service.create(input).await;

    assert!(matches!(output, Err(Error::NamespaceAlreadyExists)));
}
