pub use domain::use_case::{Error, Input, Output, UseCase};

use frontend_domain::workflow::list as domain;
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

    service.list(input).await
}

impl domain::Repository for Adapter {
    //
}

impl domain::find_workflows::Port for Adapter {
    async fn find_workflows(
        &self,
        input: domain::find_workflows::Input,
    ) -> Result<domain::find_workflows::Output, domain::find_workflows::Error> {
        let mut connection = self
            .pool
            .connect()
            .await
            .map_err(|error| domain::find_workflows::Error::Connection(error.into()))?;
        let rows = sqlx::query!(
            r#"
                SELECT
                    w.workflow_id,
                    w.namespace_id,
                    w.name,
                    w.cron,
                    w.input,
                    w.created_at
                FROM workflow w
                WHERE ($1::UUID IS NOT NULL AND w.workflow_id > $1)
                    OR ($1::UUID IS NULL)
                ORDER BY w.workflow_id ASC
                LIMIT $2
            "#,
            input.pagination.cursor(),
            input.pagination.size().as_i64(),
        )
        .fetch_all(&mut *connection)
        .await
        .map_err(|error| domain::find_workflows::Error::Connection(error.into()))?;
        let workflows = rows
            .into_iter()
            .map(|row| domain::find_workflows::Workflow {
                workflow_id: row.workflow_id,
                namespace_id: row.namespace_id,
                name: row.name,
                cron: row.cron.and_then(|cron| cron.parse().ok()),
                input: row.input.to_string(),
                created_at: row.created_at,
            })
            .collect();

        Ok(workflows)
    }
}
