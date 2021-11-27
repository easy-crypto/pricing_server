use futures::stream::TryStreamExt;
use mongodb::options::FindOptions;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use serde::{Deserialize, Serialize};

const DB_COLLECTION_NAME: &'static str = "prices";

pub struct PriceDB {
    pub name: String,
    db: Database,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    #[serde(rename = "_id")]
    pub id: chrono::DateTime<chrono::Utc>,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub vwap: f64,
    pub volume: f64,
    pub count: u32,
}

impl PriceDB {
    pub async fn new(
        db_name: &str,
        db_username: &str,
        db_hostname: &str,
        db_password: &str,
    ) -> mongodb::error::Result<Self> {
        // Parse your connection string into an options struct
        let client_options = ClientOptions::parse(format!(
            "mongodb+srv://{username}:{password}@{hostname}/{database}?w=majority",
            username = db_username,
            password = db_password,
            hostname = db_hostname,
            database = db_name
        ))
        .await?;

        warn!("db: this is a warning");

        // Get a handle to the cluster.
        let client = Client::with_options(client_options)?;

        Ok(PriceDB {
            name: "yo".to_owned(),
            db: client.database(db_name),
        })
    }

    pub async fn get_prices(&self, from: &str, limit: u8) -> mongodb::error::Result<Vec<Price>> {
        info!("getting prices");
        let price_collection = self.db.collection::<Price>(DB_COLLECTION_NAME);

        let filter = doc! {
            "_id": {
                "$gte": from,
            },
        };
        let find_options = FindOptions::builder()
            .sort(doc! { "_id": -1})
            .limit(limit as i64)
            .build();
        let cursor = price_collection.find(filter, find_options).await?;
        let v: Vec<Price> = cursor.try_collect().await?;
        Ok(v)
    }
}
