pub use domain::use_case::{Error, Input, Output, UseCase};

use frontend_domain::workflow::one as domain;
use postgresql::Database;
use sqlx::types::Uuid;

pub struct Adapter {
    pool: postgresql::ConnectionPool,
}

pub async fn execute(
    input: domain::use_case::Input,
    pool: postgresql::ConnectionPool,
) -> Result<domain::use_case::Output, domain::use_case::Error> {
    let repository = Adapter { pool };
    let service = domain::service::Service::new(repository);

    service.one(input).await
}

impl domain::Repository for Adapter {
    //
}

impl domain::find_workflow::Port for Adapter {
    async fn find_workflow(
        &self,
        input: domain::find_workflow::Input,
    ) -> Result<domain::find_workflow::Output, domain::find_workflow::Error> {
        let mut connection = self
            .pool
            .connect()
            .await
            .map_err(|error| domain::find_workflow::Error::Connection(error.into()))?;
        let row = sqlx::query!(
            r#"
                SELECT
                    w.namespace_id,
                    w.workflow_id,
                    w.name,
                    w.cron,
                    w.input,
                    w.created_at
                FROM workflow w
                WHERE w.workflow_id = $1
            "#,
            input.workflow_id,
        )
        .fetch_optional(&mut *connection)
        .await
        .map_err(|error| domain::find_workflow::Error::Connection(error.into()))?;
        let workflow = row.map(|row| domain::find_workflow::Workflow {
            namespace_id: row.namespace_id,
            workflow_id: row.workflow_id,
            name: row.name,
            cron: row.cron.and_then(|cron| cron.parse().ok()),
            input: row.input.to_string(),
            created_at: row.created_at,
        });

        Ok(workflow)
    }
}
