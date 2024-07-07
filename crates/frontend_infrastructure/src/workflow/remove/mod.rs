pub use domain::use_case::{Error, Input, Output, UseCase};

use frontend_domain::workflow::remove as domain;
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

    service.remove(input).await
}

impl domain::Repository for Adapter {
    //
}

impl domain::remove_workflow_by_id::Port for Adapter {
    async fn remove_workflow_by_id(
        &self,
        input: domain::remove_workflow_by_id::Input,
    ) -> Result<domain::remove_workflow_by_id::Output, domain::remove_workflow_by_id::Error> {
        let mut transaction = self
            .pool
            .transaction()
            .await
            .map_err(|error| domain::remove_workflow_by_id::Error::Transaction(error.into()))?;
        sqlx::query!(
            r#"
                UPDATE workflow
                SET deleted_at = NOW()
                WHERE workflow_id = $1
            "#,
            input.workflow_id,
        )
        .execute(&mut *transaction)
        .await;
        transaction
            .commit()
            .await
            .map_err(|error| domain::remove_workflow_by_id::Error::Commit(error.into()))?;

        Ok(())
    }
}
