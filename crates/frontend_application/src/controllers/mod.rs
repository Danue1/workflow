mod namespace;
mod workflow;

use rocket::{Build, Rocket};

pub fn mount(rocket: Rocket<Build>, pool: postgresql::ConnectionPool) -> Rocket<Build> {
    let rocket = namespace::mount(rocket, &pool);
    let rocket = workflow::mount(rocket, &pool);

    rocket
}
