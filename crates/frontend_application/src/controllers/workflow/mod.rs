mod list;
mod one;
mod remove;

use rocket::{Build, Rocket};

pub fn mount(rocket: Rocket<Build>, pool: &postgresql::ConnectionPool) -> Rocket<Build> {
    let service = frontend_infrastructure::workflow::WorkflowService::new(pool);
    let rocket = rocket.manage(service);

    let rocket = list::mount(rocket);
    let rocket = one::mount(rocket);
    let rocket = remove::mount(rocket);

    rocket
}
