mod create;
mod list;
mod remove;

use rocket::{Build, Rocket};

pub fn mount(rocket: Rocket<Build>, pool: &postgresql::ConnectionPool) -> Rocket<Build> {
    let service = infrastructure::namespace::NamespaceService::new(pool);
    let rocket = rocket.manage(service);
    let rocket = create::mount(rocket);
    let rocket = list::mount(rocket);
    let rocket = remove::mount(rocket);

    rocket
}
