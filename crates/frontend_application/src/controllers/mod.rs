mod namespace;

use rocket::{Build, Rocket};

pub fn mount(rocket: Rocket<Build>, pool: postgresql::ConnectionPool) -> Rocket<Build> {
    let rocket = namespace::mount(rocket, &pool);

    rocket
}
