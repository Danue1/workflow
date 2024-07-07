use new_types::Cron;
use rocket::{serde::json::Json, Build, Rocket, State};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Response {
    pub workflows: Vec<Workflow>,
}

#[derive(Serialize)]
pub struct Workflow {
    pub namespace_id: Uuid,
    pub workflow_id: Uuid,
    pub name: String,
    pub cron: Option<Cron>,
    pub input: String,
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
    let rocket = rocket.mount("/", routes![workflow_one]);

    rocket
}

#[get("/api/workflows/<workflow_id>")]
pub async fn workflow_one(
    service: &State<frontend_infrastructure::workflow::WorkflowService>,
    workflow_id: &str,
) -> Result<Json<Response>, Error> {
    let workflow_id = Uuid::parse_str(workflow_id).map_err(|_| Error::BadRequest(()))?;
    let input = frontend_infrastructure::workflow::one::Input { workflow_id };

    match service.one(input).await {
        Ok(output) => Ok(Json(Response {
            workflows: vec![Workflow {
                namespace_id: output.namespace_id,
                workflow_id: output.workflow_id,
                name: output.name,
                cron: output.cron,
                input: output.input,
                created_at: output.created_at,
            }],
        })),
        Err(frontend_infrastructure::workflow::one::Error::WorkflowNotFound) => {
            Err(Error::NotFound(()))
        }
        _ => Err(Error::InternalServerError(())),
    }
}
