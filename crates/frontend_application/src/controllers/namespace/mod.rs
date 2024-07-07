mod create;
mod list;
mod one;
mod remove;
mod task_queue__create;

use rocket::{Build, Rocket};

pub fn mount(rocket: Rocket<Build>, pool: &postgresql::ConnectionPool) -> Rocket<Build> {
    let service = frontend_infrastructure::namespace::NamespaceService::new(pool);
    let rocket = rocket.manage(service);

    let rocket = create::mount(rocket);
    let rocket = list::mount(rocket);
    let rocket = one::mount(rocket);
    let rocket = remove::mount(rocket);
    let rocket = task_queue__create::mount(rocket);

    rocket
}
