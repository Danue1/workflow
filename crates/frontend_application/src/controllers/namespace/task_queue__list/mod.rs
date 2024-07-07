use ::domain::TaskQueueType;
use frontend_infrastructure::namespace::task_queue__list as domain;
use new_types::Pagination;
use rocket::{serde::json::Json, Build, Rocket, State};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Response {
    pub task_queues: Vec<TaskQueue>,
}

#[derive(Serialize)]
pub struct TaskQueue {
    pub task_queue_id: Uuid,
    pub name: String,
    pub r#type: TaskQueueType,
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
    let rocket = rocket.mount("/", routes![task_queue__list]);

    rocket
}

#[get("/api/namespaces/<namespace_id>/task_queues?<cursor>&<size>")]
pub async fn task_queue__list(
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

    match service.task_queue__list(input).await {
        Ok(output) => Ok(Json(Response {
            task_queues: output
                .into_iter()
                .map(|task_queue| TaskQueue {
                    task_queue_id: task_queue.task_queue_id,
                    name: task_queue.name,
                    r#type: task_queue.r#type,
                    created_at: task_queue.created_at,
                })
                .collect(),
        })),
        _ => Err(Error::InternalServerError(())),
    }
}
