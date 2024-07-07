use super::*;
use uuid::Uuid;

const NAMESPACE_ID: Uuid = Uuid::nil();
const WORKFLOW_ID: Uuid = Uuid::nil();

#[tokio::test]
async fn success() {
    struct StubRepository;

    impl Repository for StubRepository {
        //
    }

    impl find_namespace_by_id::Port for StubRepository {
        async fn find_namespace_by_id(
            &self,
            input: find_namespace_by_id::Input,
        ) -> Result<find_namespace_by_id::Output, find_namespace_by_id::Error> {
            Ok(Some(find_namespace_by_id::Namespace {
                namespace_id: NAMESPACE_ID,
            }))
        }
    }

    impl find_workflow_by_name::Port for StubRepository {
        async fn find_workflow_by_name(
            &self,
            input: find_workflow_by_name::Input,
        ) -> Result<find_workflow_by_name::Output, find_workflow_by_name::Error> {
            Ok(None)
        }
    }

    impl create_workflow::Port for StubRepository {
        async fn create_workflow(
            &self,
            input: create_workflow::Input,
        ) -> Result<create_workflow::Output, create_workflow::Error> {
            Ok(create_workflow::Output {
                workflow_id: WORKFLOW_ID,
            })
        }
    }

    let repository = StubRepository;
    let service = Service::new(repository);
    let input = Input {
        namespace_id: NAMESPACE_ID,
        name: "name".to_string(),
        cron: None,
        input: "{}".to_owned(),
    };
    let output = service.workflow__create(input).await;

    assert!(matches!(
        output,
        Ok(Output {
            workflow_id: WORKFLOW_ID
        })
    ));
}

#[tokio::test]
async fn namespace_not_found() {
    struct StubRepository;

    impl Repository for StubRepository {
        //
    }

    impl find_namespace_by_id::Port for StubRepository {
        async fn find_namespace_by_id(
            &self,
            input: find_namespace_by_id::Input,
        ) -> Result<find_namespace_by_id::Output, find_namespace_by_id::Error> {
            Ok(None)
        }
    }

    impl find_workflow_by_name::Port for StubRepository {
        //
    }

    impl create_workflow::Port for StubRepository {
        //
    }

    let repository = StubRepository;
    let service = Service::new(repository);
    let input = Input {
        namespace_id: NAMESPACE_ID,
        name: "name".to_string(),
        cron: None,
        input: "{}".to_owned(),
    };
    let output = service.workflow__create(input).await;

    assert!(matches!(output, Err(Error::NamespaceNotFound)));
}

#[tokio::test]
async fn workflow_already_exists() {
    struct StubRepository;

    impl Repository for StubRepository {
        //
    }

    impl find_namespace_by_id::Port for StubRepository {
        async fn find_namespace_by_id(
            &self,
            input: find_namespace_by_id::Input,
        ) -> Result<find_namespace_by_id::Output, find_namespace_by_id::Error> {
            Ok(Some(find_namespace_by_id::Namespace {
                namespace_id: NAMESPACE_ID,
            }))
        }
    }

    impl find_workflow_by_name::Port for StubRepository {
        async fn find_workflow_by_name(
            &self,
            input: find_workflow_by_name::Input,
        ) -> Result<find_workflow_by_name::Output, find_workflow_by_name::Error> {
            Ok(Some(find_workflow_by_name::Workflow {
                workflow_id: WORKFLOW_ID,
            }))
        }
    }

    impl create_workflow::Port for StubRepository {
        //
    }

    let repository = StubRepository;
    let service = Service::new(repository);
    let input = Input {
        namespace_id: NAMESPACE_ID,
        name: "name".to_string(),
        cron: None,
        input: "{}".to_owned(),
    };
    let output = service.workflow__create(input).await;

    assert!(matches!(output, Err(Error::WorkflowAlreadyExists)));
}
