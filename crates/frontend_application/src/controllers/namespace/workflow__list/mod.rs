use frontend_infrastructure::namespace::workflow__list as domain;
use new_types::Pagination;
use rocket::{serde::json::Json, Build, Rocket, State};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Response {
    pub workflows: Vec<Workflow>,
}

#[derive(Serialize)]
pub struct Workflow {
    pub workflow_id: Uuid,
    pub namespace_id: Uuid,
    pub name: String,
    pub cron: Option<String>,
    pub input: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Responder)]
pub enum Error {
    #[response(status = 400)]
    BadRequest(()),
    #[response(status = 500)]
    InternalServerError(()),
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = rocket.mount("/", routes![workflow__list]);

    rocket
}

#[get("/api/namespaces/<namespace_id>/workflows?<cursor>&<size>")]
pub async fn workflow__list(
    service: &State<frontend_infrastructure::namespace::NamespaceService>,
    namespace_id: &str,
    cursor: Option<&str>,
    size: Option<usize>,
) -> Result<Json<Response>, Error> {
    let namespace_id = Uuid::parse_str(namespace_id).map_err(|_| Error::BadRequest(()))?;
    let cursor = match cursor {
        Some(cursor) if cursor.trim().is_empty() => None,
        Some(cursor) => Some(Uuid::parse_str(cursor).map_err(|_| Error::BadRequest(()))?),
        None => None,
    };
    let input = domain::Input {
        namespace_id,
        pagination: Pagination::from((cursor, size)),
    };

    match service.workflow__list(input).await {
        Ok(output) => Ok(Json(Response {
            workflows: output
                .into_iter()
                .map(|workflow| Workflow {
                    workflow_id: workflow.workflow_id,
                    namespace_id: workflow.namespace_id,
                    name: workflow.name,
                    cron: workflow.cron.map(|cron| cron.source()),
                    input: workflow.input,
                    created_at: workflow.created_at,
                })
                .collect(),
        })),
        _ => Err(Error::InternalServerError(())),
    }
}
