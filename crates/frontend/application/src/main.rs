#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde;

mod app;
mod config;
mod controllers;

pub use config::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::create().await?.launch().await?;

    Ok(())
}
