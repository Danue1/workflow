mod setup;

use crate::controllers;
use rocket::{Build, Rocket};

pub async fn create() -> Result<Rocket<Build>, Box<dyn std::error::Error>> {
    let rocket = Rocket::build();
    let rocket = setup::configure(rocket);

    let pool = postgresql::ConnectionPool::new(crate::DATABASE_URL).await?;
    let rocket = controllers::mount(rocket, pool);

    Ok(rocket)
}
