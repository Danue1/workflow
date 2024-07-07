use ::domain::TaskQueueType;
use frontend_infrastructure::namespace::task_queue__create as domain;
use rocket::{serde::json::Json, Build, Rocket, State};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Body {
    pub name: String,
    pub r#type: TaskQueueType,
}

#[derive(Serialize)]
pub struct Response {
    pub task_queue_id: Uuid,
}

#[derive(Debug, Responder)]
pub enum Error {
    #[response(status = 404)]
    NotFound(()),
    #[response(status = 409)]
    Conflict(()),
    #[response(status = 500)]
    InternalServerError(()),
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = rocket.mount("/", routes![task_queue_create]);

    rocket
}

#[post("/api/namespaces/<namespace_id>/task_queues", data = "<body>")]
pub async fn task_queue_create(
    service: &State<frontend_infrastructure::namespace::NamespaceService>,
    namespace_id: &str,
    body: Json<Body>,
) -> Result<Json<Response>, Error> {
    let namespace_id = Uuid::parse_str(namespace_id).map_err(|_| Error::NotFound(()))?;
    let body = body.into_inner();
    let input = domain::Input {
        namespace_id,
        name: body.name,
        r#type: body.r#type,
    };

    match service.task_queue__create(input).await {
        Ok(output) => Ok(Json(Response {
            task_queue_id: output.task_queue_id,
        })),
        Err(domain::Error::NamespaceNotFound) => Err(Error::NotFound(())),
        Err(domain::Error::TaskQueueAlreadyExists) => Err(Error::Conflict(())),
        _ => Err(Error::InternalServerError(())),
    }
}
