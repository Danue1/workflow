pub use domain::use_case::{Error, Input, Output, UseCase};

use frontend_domain::namespace::task_queue__list as domain;
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

    service.task_queue__list(input).await
}

impl domain::Repository for Adapter {
    //
}

impl domain::find_task_queues::Port for Adapter {
    async fn find_task_queues(
        &self,
        input: domain::find_task_queues::Input,
    ) -> Result<domain::find_task_queues::Output, domain::find_task_queues::Error> {
        let mut connection = self
            .pool
            .connect()
            .await
            .map_err(|error| domain::find_task_queues::Error::Connection(error.into()))?;
        let rows = sqlx::query!(
            r#"
                SELECT
                    tq.task_queue_id,
                    tq.name,
                    tq.type,
                    tq.created_at
                FROM task_queue tq
                WHERE tq.namespace_id = $1
                    AND (
                        ($2::UUID IS NOT NULL AND tq.task_queue_id > $2)
                        OR ($2::UUID IS NULL)
                    )
                ORDER BY tq.task_queue_id ASC
                LIMIT $3
            "#,
            input.namespace_id,
            input.pagination.cursor(),
            input.pagination.size().as_i64(),
        )
        .fetch_all(&mut *connection)
        .await
        .map_err(|error| domain::find_task_queues::Error::Connection(error.into()))?;
        let task_queues = rows
            .into_iter()
            .map(|row| domain::find_task_queues::TaskQueue {
                task_queue_id: row.task_queue_id,
                name: row.name,
                r#type: row.r#type.into(),
                created_at: row.created_at,
            })
            .collect();

        Ok(task_queues)
    }
}
