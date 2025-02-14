use frontend_infrastructure::namespace::remove as domain;
use rocket::{response::status::NoContent, Build, Rocket, State};

#[derive(Debug, Responder)]
pub enum Error {
    #[response(status = 400)]
    BadRequest(()),
    #[response(status = 500)]
    InternalServerError(()),
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = rocket.mount("/", routes![namespace_remove]);

    rocket
}

#[delete("/api/namespaces/<namespace_id>")]
pub async fn namespace_remove(
    service: &State<frontend_infrastructure::namespace::NamespaceService>,
    namespace_id: &str,
) -> Result<NoContent, Error> {
    let namespace_id = namespace_id.parse().map_err(|_| Error::BadRequest(()))?;
    let input = domain::Input { namespace_id };

    match service.remove(input).await {
        Ok(_) => Ok(NoContent),
        _ => Err(Error::InternalServerError(())),
    }
}
