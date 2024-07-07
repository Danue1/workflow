pub use domain::use_case::{Error, Input, Output, UseCase};

use frontend_domain::namespace::one as domain;
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

impl domain::find_namespace::Port for Adapter {
    async fn find_namespace(
        &self,
        input: domain::find_namespace::Input,
    ) -> Result<domain::find_namespace::Output, domain::find_namespace::Error> {
        let mut connection = self
            .pool
            .connect()
            .await
            .map_err(|error| domain::find_namespace::Error::Connection(error.into()))?;
        let row = sqlx::query!(
            r#"
                SELECT
                    n.namespace_id,
                    n.name,
                    n.created_at
                FROM namespace n
                WHERE n.namespace_id = $1
            "#,
            input.namespace_id,
        )
        .fetch_optional(&mut *connection)
        .await
        .map_err(|error| domain::find_namespace::Error::Connection(error.into()))?;
        let namespace = row.map(|row| domain::find_namespace::Namespace {
            namespace_id: row.namespace_id,
            name: row.name,
            created_at: row.created_at,
        });

        Ok(namespace)
    }
}
