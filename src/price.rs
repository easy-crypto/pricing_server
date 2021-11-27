use crate::{price_db::Price, PriceDB};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::State;

#[derive(Serialize)]
pub struct PricesRespone {
    next: Option<chrono::DateTime<chrono::Utc>>,
    prices: Vec<Price>,
}

#[get("/prices?<from>&<limit>")]
pub async fn prices(from: &str, limit: u8, db: &State<PriceDB>) -> Option<Json<PricesRespone>> {
    let prices = match db.get_prices(from, limit).await {
        Ok(v) => v,
        Err(e) => {
            error!("Fail to get prices. {:?}", e);
            return None;
        }
    };

    let next: Option<chrono::DateTime<chrono::Utc>> = match prices.last() {
        Some(n) => Some(n.id),
        None => None,
    };

    Some(Json(PricesRespone { next, prices }))
}
