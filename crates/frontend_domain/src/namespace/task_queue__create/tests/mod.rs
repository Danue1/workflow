use super::*;
use domain::TaskQueueType;
use uuid::Uuid;

const NAMESPACE_ID: Uuid = Uuid::nil();
const TASK_QUEUE_ID: Uuid = Uuid::nil();

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

    impl find_task_queue_by_name::Port for StubRepository {
        async fn find_task_queue_by_name(
            &self,
            input: find_task_queue_by_name::Input,
        ) -> Result<find_task_queue_by_name::Output, find_task_queue_by_name::Error> {
            Ok(None)
        }
    }

    impl create_task_queue::Port for StubRepository {
        async fn create_task_queue(
            &self,
            input: create_task_queue::Input,
        ) -> Result<create_task_queue::Output, create_task_queue::Error> {
            Ok(create_task_queue::Output {
                task_queue_id: TASK_QUEUE_ID,
            })
        }
    }

    let repository = StubRepository;
    let service = Service::new(repository);
    let input = Input {
        namespace_id: NAMESPACE_ID,
        name: "name".to_string(),
        r#type: TaskQueueType::Workflow,
    };
    let output = service.task_queue__create(input).await;

    assert!(matches!(
        output,
        Ok(Output {
            task_queue_id: TASK_QUEUE_ID
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

    impl find_task_queue_by_name::Port for StubRepository {
        //
    }

    impl create_task_queue::Port for StubRepository {
        //
    }

    let repository = StubRepository;
    let service = Service::new(repository);
    let input = Input {
        namespace_id: NAMESPACE_ID,
        name: "name".to_string(),
        r#type: TaskQueueType::Workflow,
    };
    let output = service.task_queue__create(input).await;

    assert!(matches!(output, Err(Error::NamespaceNotFound)));
}

#[tokio::test]
async fn task_queue_already_exists() {
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

    impl find_task_queue_by_name::Port for StubRepository {
        async fn find_task_queue_by_name(
            &self,
            input: find_task_queue_by_name::Input,
        ) -> Result<find_task_queue_by_name::Output, find_task_queue_by_name::Error> {
            Ok(Some(find_task_queue_by_name::TaskQueue {
                task_queue_id: TASK_QUEUE_ID,
            }))
        }
    }

    impl create_task_queue::Port for StubRepository {
        //
    }

    let repository = StubRepository;
    let service = Service::new(repository);
    let input = Input {
        namespace_id: NAMESPACE_ID,
        name: "name".to_string(),
        r#type: TaskQueueType::Workflow,
    };
    let output = service.task_queue__create(input).await;

    assert!(matches!(output, Err(Error::TaskQueueAlreadyExists)));
}
