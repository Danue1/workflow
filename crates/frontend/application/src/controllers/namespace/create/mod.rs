use infrastructure::namespace::create as domain;
use rocket::{serde::json::Json, Build, Rocket, State};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Body {
    pub name: String,
}

#[derive(Serialize)]
pub struct Response {
    pub namespace_id: Uuid,
}

#[derive(Debug, Responder)]
pub enum Error {
    #[response(status = 409)]
    Conflict(()),
    #[response(status = 500)]
    InternalServerError(()),
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = rocket.mount("/", routes![namespace_create]);

    rocket
}

#[post("/api/namespaces", data = "<body>")]
pub async fn namespace_create(
    service: &State<infrastructure::namespace::NamespaceService>,
    body: Json<Body>,
) -> Result<Json<Response>, Error> {
    let body = body.into_inner();
    let input = domain::Input { name: body.name };

    match service.create(input).await {
        Ok(output) => Ok(Json(Response {
            namespace_id: output.namespace_id,
        })),
        Err(domain::Error::NamespaceAlreadyExists) => Err(Error::Conflict(())),
        _ => Err(Error::InternalServerError(())),
    }
}
