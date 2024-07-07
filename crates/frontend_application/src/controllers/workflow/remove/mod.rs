use frontend_infrastructure::workflow::remove as domain;
use rocket::{response::status::NoContent, Build, Rocket, State};

#[derive(Debug, Responder)]
pub enum Error {
    #[response(status = 400)]
    BadRequest(()),
    #[response(status = 500)]
    InternalServerError(()),
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = rocket.mount("/", routes![workflow_remove]);

    rocket
}

#[delete("/api/workflows/<workflow_id>")]
pub async fn workflow_remove(
    service: &State<frontend_infrastructure::workflow::WorkflowService>,
    workflow_id: &str,
) -> Result<NoContent, Error> {
    let workflow_id = workflow_id.parse().map_err(|_| Error::BadRequest(()))?;
    let input = domain::Input { workflow_id };

    match service.remove(input).await {
        Ok(_) => Ok(NoContent),
        _ => Err(Error::InternalServerError(())),
    }
}
