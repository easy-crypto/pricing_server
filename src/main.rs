//#[macro_use]
//extern crate rocket;
//
//#[get("/")]
//fn index() -> &'static str {
//    "Hello, world!"
//}
//
//fn main() -> Result<(), Box<dyn Error>> {
//    dotenv().ok();
//    let port = env::var("PORT").ok();
//    println!("Listening on {}", port.unwrap_or("2000".to_owned()));
//
//    let config = Config::build(Environment::Staging)
//        .address("1.2.3.4")
//        .port(9234)
//        .finalize()?;
//
//    rocket::custom(config).mount("/", routes![index]).launch();
//}
//
#[macro_use]
extern crate rocket;

use std::env;

use dotenv::dotenv;
use rocket::serde::{Deserialize, Serialize};

use rocket::fairing::AdHoc;

use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment, Profile,
};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    port: u16,
    /* and so on.. */
}

impl Default for Config {
    fn default() -> Config {
        dotenv().ok();

        let port = env::var("PORT").ok();
        let port: u16 = port
            .expect("PORT not set")
            .parse()
            .expect("PORT should be number");

        Config {
            port: u16::from(port),
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(Config::default()))
        .merge(Toml::file("App.toml").nested())
        .merge(Env::prefixed("APP_").global())
        .select(Profile::from_env_or("APP_PROFILE", "default"));

    rocket::custom(figment)
        .mount("/", routes![index])
        .attach(AdHoc::config::<Config>())
}
