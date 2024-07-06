pub use domain::use_case::{Error, Input, Output, UseCase};

use domain::namespace::create as domain;
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

    service.create(input).await
}

impl domain::Repository for Adapter {
    //
}

impl domain::find_namespace_by_name::Port for Adapter {
    async fn find_namespace_by_name(
        &self,
        input: domain::find_namespace_by_name::Input,
    ) -> Result<domain::find_namespace_by_name::Output, domain::find_namespace_by_name::Error> {
        let mut connection = self
            .pool
            .connect()
            .await
            .map_err(|error| domain::find_namespace_by_name::Error::Connection(error.into()))?;
        let row = sqlx::query!(
            r#"
                SELECT n.namespace_id
                FROM namespace n
                WHERE n.name = $1
            "#,
            input.name,
        )
        .fetch_optional(&mut *connection)
        .await;

        match row {
            Ok(Some(row)) => Ok(Some(domain::find_namespace_by_name::Namespace {
                id: row.namespace_id,
            })),
            Ok(None) => Ok(None),
            Err(error) => Err(domain::find_namespace_by_name::Error::Connection(
                error.into(),
            )),
        }
    }
}

impl domain::create_namespace::Port for Adapter {
    async fn create_namespace(
        &self,
        input: domain::create_namespace::Input,
    ) -> Result<domain::create_namespace::Output, domain::create_namespace::Error> {
        let mut transaction = self
            .pool
            .transaction()
            .await
            .map_err(|error| domain::create_namespace::Error::Transaction(error.into()))?;
        let namespace_id = Uuid::now_v7();
        sqlx::query!(
            r#"
                INSERT INTO namespace (namespace_id, name)
                VALUES ($1, $2)
            "#,
            namespace_id,
            input.name,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|error| domain::create_namespace::Error::Transaction(error.into()))?;
        transaction
            .commit()
            .await
            .map_err(|error| domain::create_namespace::Error::Commit(error.into()))?;

        Ok(domain::create_namespace::Output { namespace_id })
    }
}
