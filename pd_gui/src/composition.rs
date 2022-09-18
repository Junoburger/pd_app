use serde::{Deserialize, Serialize};

use crate::artist::Artist;

#[derive(Serialize, Deserialize, Debug)]
pub struct Composition {
    pub id: i64,
    pub artist: String,
    pub title: String,
    pub desc: String,
    pub sounds: Vec<i64>,
    pub collaborators: Vec<Artist>,
}

#[tokio::main]
pub async fn get_compositions() -> Result<Vec<Composition>, Box<dyn std::error::Error>> {
    let compositions: Vec<Composition> = reqwest::get("http://127.0.0.1:8000/compositions")
        .await?
        .json()
        .await?;

    Ok(compositions)
}
