use rocket::{serde::json::Json, Build, Rocket, State};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Response {
    pub namespaces: Namespace,
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
    #[response(status = 404)]
    NotFound(()),
    #[response(status = 500)]
    InternalServerError(()),
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = rocket.mount("/", routes![namespace_one]);

    rocket
}

#[get("/api/namespaces/<namespace_id>")]
pub async fn namespace_one(
    service: &State<frontend_infrastructure::namespace::NamespaceService>,
    namespace_id: &str,
) -> Result<Json<Response>, Error> {
    let namespace_id = Uuid::parse_str(namespace_id).map_err(|_| Error::BadRequest(()))?;
    let input = frontend_infrastructure::namespace::one::Input { namespace_id };

    match service.one(input).await {
        Ok(output) => Ok(Json(Response {
            namespaces: Namespace {
                namespace_id: output.namespace_id,
                name: output.name,
                created_at: output.created_at,
            },
        })),
        Err(frontend_infrastructure::namespace::one::Error::NamespaceNotFound) => {
            Err(Error::NotFound(()))
        }
        _ => Err(Error::InternalServerError(())),
    }
}
