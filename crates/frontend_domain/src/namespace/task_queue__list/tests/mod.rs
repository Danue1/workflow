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

    impl find_task_queues::Port for StubRepository {
        async fn find_task_queues(
            &self,
            input: find_task_queues::Input,
        ) -> Result<find_task_queues::Output, find_task_queues::Error> {
            Ok(vec![])
        }
    }

    let repository = StubRepository;
    let service = Service::new(repository);
    let input = Input {
        namespace_id: NAMESPACE_ID,
        pagination: Pagination::from(10),
    };
    let output = service.task_queue__list(input).await;

    assert!(matches!(output.as_deref(), Ok(&[])));
}
