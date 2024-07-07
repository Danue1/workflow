use super::*;
use uuid::Uuid;

const NAMESPACE_ID: Uuid = Uuid::nil();

#[tokio::test]
async fn found() {
    struct StubRepository;

    impl Repository for StubRepository {
        //
    }

    impl find_workflow::Port for StubRepository {
        async fn find_workflow(
            &self,
            input: find_workflow::Input,
        ) -> Result<find_workflow::Output, find_workflow::Error> {
            Ok(Some(find_workflow::Workflow {
                namespace_id: input.workflow_id,
                workflow_id: input.workflow_id,
                name: "workflow".to_owned(),
                cron: None,
                input: "{}".to_owned(),
                created_at: chrono::DateTime::<chrono::Utc>::MIN_UTC,
            }))
        }
    }

    let repository = StubRepository;
    let service = Service::new(repository);
    let input = Input {
        workflow_id: NAMESPACE_ID,
    };
    let output = service.one(input).await;

    assert!(matches!(
        output,
        Ok(Workflow {
            namespace_id: NAMESPACE_ID,
            workflow_id,
            name,
            cron,
            input,
            created_at,
        })
    ));
}
