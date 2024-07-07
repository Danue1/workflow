use frontend_infrastructure::namespace::workflow__create as domain;
use rocket::{serde::json::Json, Build, Rocket, State};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Body {
    pub name: String,
    pub cron: Option<String>,
    pub input: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    pub workflow_id: Uuid,
}

#[derive(Debug, Responder)]
pub enum Error {
    #[response(status = 400)]
    BadRequest(()),
    #[response(status = 404)]
    NotFound(()),
    #[response(status = 409)]
    Conflict(()),
    #[response(status = 500)]
    InternalServerError(()),
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = rocket.mount("/", routes![workflow__create]);

    rocket
}

#[post("/api/namespaces/<namespace_id>/workflows", data = "<body>")]
pub async fn workflow__create(
    service: &State<frontend_infrastructure::namespace::NamespaceService>,
    namespace_id: &str,
    body: Json<Body>,
) -> Result<Json<Response>, Error> {
    let namespace_id = Uuid::parse_str(namespace_id).map_err(|_| Error::NotFound(()))?;
    let body = body.into_inner();
    let input = frontend_infrastructure::namespace::workflow__create::Input {
        namespace_id,
        name: body.name,
        cron: match body.cron {
            Some(cron) => Some(cron.parse().map_err(|_| Error::BadRequest(()))?),
            None => None,
        },
        input: match body.input {
            Some(input) => input,
            None => "{}".to_string(),
        },
    };

    match service.workflow__create(input).await {
        Ok(output) => Ok(Json(Response {
            workflow_id: output.workflow_id,
        })),
        Err(domain::Error::NamespaceNotFound) => Err(Error::NotFound(())),
        Err(domain::Error::WorkflowAlreadyExists) => Err(Error::Conflict(())),
        _ => Err(Error::InternalServerError(())),
    }
}
