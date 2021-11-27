#[macro_use]
extern crate rocket;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod price;
mod price_db;

use std::env;

use dotenv::dotenv;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;

use rocket::fairing::AdHoc;
use rocket::serde::json::Json;

use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment, Profile,
};

use price::prices;
use price_db::{Price, PriceDB};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Config {
    port: u16,
    address: String,
    db_name: String,
    db_username: String,
    db_hostname: String,
    db_password: String,
}

impl Default for Config {
    fn default() -> Config {
        dotenv().ok();

        let port: u16 = env::var("PORT")
            .ok()
            .expect("PORT needed")
            .parse()
            .expect("PORT should be number");
        let address: String = env::var("ADDRESS").ok().expect("ADDRESS needed");
        let db_name = env::var("DB_NAME").ok().expect("DB_NAME needed");
        let db_username = env::var("DB_USERNAME").ok().expect("DB_USERNAME needed");
        let db_hostname = env::var("DB_HOSTNAME").ok().expect("DB_HOSTNAME needed");
        let db_password = env::var("DB_PASSWORD").ok().expect("DB_PASSWORD needed");

        Config {
            port: u16::from(port),
            address,
            db_name,
            db_username,
            db_hostname,
            db_password,
        }
    }
}

#[launch]
async fn rocket() -> _ {
    info!("such information");

    let config = Config::default();

    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(config.to_owned()))
        .merge(Toml::file("App.toml").nested())
        .merge(Env::prefixed("APP_").global())
        .select(Profile::from_env_or("APP_PROFILE", "default"));

    let db: PriceDB = PriceDB::new(
        &config.db_name,
        &config.db_username,
        &config.db_hostname,
        &config.db_password,
    )
    .await
    .ok()
    .expect("price db connection is ok");

    rocket::custom(figment)
        .manage(db)
        .mount("/", routes![prices])
        .attach(AdHoc::config::<Config>())
}
