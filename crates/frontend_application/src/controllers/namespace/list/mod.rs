use frontend_infrastructure::namespace::list as domain;
use new_types::Pagination;
use rocket::{serde::json::Json, Build, Rocket, State};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Response {
    pub namespaces: Vec<Namespace>,
}

#[derive(Serialize)]
pub struct Namespace {
    pub namespace_id: Uuid,
    pub name: String,
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
    let rocket = rocket.mount("/", routes![namespace_list]);

    rocket
}

#[get("/api/namespaces?<cursor>&<limit>")]
pub async fn namespace_list(
    service: &State<frontend_infrastructure::namespace::NamespaceService>,
    cursor: Option<&str>,
    limit: Option<usize>,
) -> Result<Json<Response>, Error> {
    let cursor = match cursor {
        Some(cursor) if cursor.trim().is_empty() => None,
        Some(cursor) => Some(Uuid::parse_str(cursor).map_err(|_| Error::BadRequest(()))?),
        None => None,
    };
    let input = domain::Input {
        pagination: Pagination::from((cursor, limit)),
    };

    match service.list(input).await {
        Ok(output) => Ok(Json(Response {
            namespaces: output
                .into_iter()
                .map(|namespace| Namespace {
                    namespace_id: namespace.namespace_id,
                    name: namespace.name,
                    created_at: namespace.created_at,
                })
                .collect(),
        })),
        _ => Err(Error::InternalServerError(())),
    }
}
