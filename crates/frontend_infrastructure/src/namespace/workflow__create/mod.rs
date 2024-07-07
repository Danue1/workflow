pub use domain::use_case::{Error, Input, Output, UseCase};

use frontend_domain::namespace::workflow__create as domain;
use postgresql::Database;
use sqlx::types::{Json, JsonValue, Uuid};

pub struct Adapter {
    pool: postgresql::ConnectionPool,
}

pub async fn execute(
    input: domain::use_case::Input,
    pool: postgresql::ConnectionPool,
) -> Result<domain::use_case::Output, domain::use_case::Error> {
    let repository = Adapter { pool };
    let service = domain::service::Service::new(repository);

    service.workflow__create(input).await
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

impl domain::find_workflow_by_name::Port for Adapter {
    async fn find_workflow_by_name(
        &self,
        input: domain::find_workflow_by_name::Input,
    ) -> Result<domain::find_workflow_by_name::Output, domain::find_workflow_by_name::Error> {
        let mut connection = self
            .pool
            .connect()
            .await
            .map_err(|error| domain::find_workflow_by_name::Error::Connection(error.into()))?;
        let row = sqlx::query!(
            r#"
                SELECT w.workflow_id
                FROM workflow w
                WHERE w.name = $1
            "#,
            input.name,
        )
        .fetch_optional(&mut *connection)
        .await;

        match row {
            Ok(Some(row)) => Ok(Some(domain::find_workflow_by_name::Workflow {
                workflow_id: row.workflow_id,
            })),
            Ok(None) => Ok(None),
            Err(error) => Err(domain::find_workflow_by_name::Error::Connection(
                error.into(),
            )),
        }
    }
}

impl domain::create_workflow::Port for Adapter {
    async fn create_workflow(
        &self,
        input: domain::create_workflow::Input,
    ) -> Result<domain::create_workflow::Output, domain::create_workflow::Error> {
        let mut transaction = self
            .pool
            .transaction()
            .await
            .map_err(|error| domain::create_workflow::Error::Transaction(error.into()))?;
        let workflow_id = Uuid::now_v7();
        let namespace_id = input.namespace_id;
        let name = input.name;
        let cron = input.cron.map(|cron| cron.source());
        let input = JsonValue::String(input.input);
        sqlx::query!(
            r#"
                INSERT INTO workflow (workflow_id, namespace_id, name, cron, input)
                VALUES ($1, $2, $3, $4, $5)
            "#,
            workflow_id,
            namespace_id,
            name,
            cron,
            input,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|error| domain::create_workflow::Error::Transaction(error.into()))?;
        transaction
            .commit()
            .await
            .map_err(|error| domain::create_workflow::Error::Commit(error.into()))?;

        Ok(domain::create_workflow::Output { workflow_id })
    }
}
