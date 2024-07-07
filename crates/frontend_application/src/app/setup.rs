use crate::PORT;
use rocket::{Build, Config, Rocket};

pub fn configure(rocket: Rocket<Build>) -> Rocket<Build> {
    let rocket = rocket.configure(Config {
        port: PORT,
        ..Config::default()
    });

    rocket
}
