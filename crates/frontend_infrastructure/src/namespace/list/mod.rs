pub use domain::use_case::{Error, Input, Output, UseCase};

use frontend_domain::namespace::list as domain;
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

impl domain::find_namespaces::Port for Adapter {
    async fn find_namespaces(
        &self,
        input: domain::find_namespaces::Input,
    ) -> Result<domain::find_namespaces::Output, domain::find_namespaces::Error> {
        let mut connection = self
            .pool
            .connect()
            .await
            .map_err(|error| domain::find_namespaces::Error::Connection(error.into()))?;
        let rows = sqlx::query!(
            r#"
                SELECT
                    n.namespace_id,
                    n.name,
                    n.created_at
                FROM namespace n
                WHERE n.namespace_id = $1
                ORDER BY n.namespace_id ASC
                LIMIT $2
            "#,
            input.pagination.cursor(),
            input.pagination.size().as_i64(),
        )
        .fetch_all(&mut *connection)
        .await
        .map_err(|error| domain::find_namespaces::Error::Connection(error.into()))?;
        let namespaces = rows
            .into_iter()
            .map(|row| domain::find_namespaces::Namespace {
                namespace_id: row.namespace_id,
                name: row.name,
                created_at: row.created_at,
            })
            .collect();

        Ok(namespaces)
    }
}
