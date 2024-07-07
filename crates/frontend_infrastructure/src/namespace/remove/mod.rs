pub use domain::use_case::{Error, Input, Output, UseCase};

use frontend_domain::namespace::remove as domain;
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

impl domain::remove_namespace_by_id::Port for Adapter {
    async fn remove_namespace_by_id(
        &self,
        input: domain::remove_namespace_by_id::Input,
    ) -> Result<domain::remove_namespace_by_id::Output, domain::remove_namespace_by_id::Error> {
        let mut transaction =
            self.pool.transaction().await.map_err(|error| {
                domain::remove_namespace_by_id::Error::Transaction(error.into())
            })?;
        sqlx::query!(
            r#"
                DELETE FROM namespace
                WHERE namespace_id = $1
            "#,
            input.namespace_id,
        )
        .execute(&mut *transaction)
        .await;
        transaction
            .commit()
            .await
            .map_err(|error| domain::remove_namespace_by_id::Error::Commit(error.into()))?;

        Ok(())
    }
}
