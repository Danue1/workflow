pub use domain::use_case::{Error, Input, Output, UseCase};

use frontend_domain::namespace::task_queue__create as domain;
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

    service.task_queue__create(input).await
}

impl domain::Repository for Adapter {
    //
}

impl domain::find_namespace_by_id::Port for Adapter {
    async fn find_namespace_by_id(
        &self,
        input: domain::find_namespace_by_id::Input,
    ) -> Result<domain::find_namespace_by_id::Output, domain::find_namespace_by_id::Error> {
        let mut connection = self
            .pool
            .connect()
            .await
            .map_err(|error| domain::find_namespace_by_id::Error::Connection(error.into()))?;
        let row = sqlx::query!(
            r#"
                SELECT n.namespace_id
                FROM namespace n
                WHERE n.namespace_id = $1
            "#,
            input.namespace_id,
        )
        .fetch_optional(&mut *connection)
        .await;

        match row {
            Ok(Some(row)) => Ok(Some(domain::find_namespace_by_id::Namespace {
                namespace_id: row.namespace_id,
            })),
            Ok(None) => Ok(None),
            Err(error) => Err(domain::find_namespace_by_id::Error::Connection(
                error.into(),
            )),
        }
    }
}

impl domain::find_task_queue_by_name::Port for Adapter {
    async fn find_task_queue_by_name(
        &self,
        input: domain::find_task_queue_by_name::Input,
    ) -> Result<domain::find_task_queue_by_name::Output, domain::find_task_queue_by_name::Error>
    {
        let mut connection =
            self.pool.connect().await.map_err(|error| {
                domain::find_task_queue_by_name::Error::Connection(error.into())
            })?;
        let row = sqlx::query!(
            r#"
                SELECT tq.task_queue_id
                FROM task_queue tq
                WHERE tq.name = $1
            "#,
            input.name,
        )
        .fetch_optional(&mut *connection)
        .await;

        match row {
            Ok(Some(row)) => Ok(Some(domain::find_task_queue_by_name::TaskQueue {
                task_queue_id: row.task_queue_id,
            })),
            Ok(None) => Ok(None),
            Err(error) => Err(domain::find_task_queue_by_name::Error::Connection(
                error.into(),
            )),
        }
    }
}

impl domain::create_task_queue::Port for Adapter {
    async fn create_task_queue(
        &self,
        input: domain::create_task_queue::Input,
    ) -> Result<domain::create_task_queue::Output, domain::create_task_queue::Error> {
        let mut transaction = self
            .pool
            .transaction()
            .await
            .map_err(|error| domain::create_task_queue::Error::Transaction(error.into()))?;
        let task_queue_id = Uuid::now_v7();
        sqlx::query!(
            r#"
                INSERT INTO task_queue (task_queue_id, namespace_id, name, type)
                VALUES ($1, $2, $3, $4)
            "#,
            task_queue_id,
            input.namespace_id,
            input.name,
            input.r#type.as_str(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(|error| domain::create_task_queue::Error::Transaction(error.into()))?;

        transaction
            .commit()
            .await
            .map_err(|error| domain::create_task_queue::Error::Commit(error.into()))?;

        Ok(domain::create_task_queue::Output { task_queue_id })
    }
}
