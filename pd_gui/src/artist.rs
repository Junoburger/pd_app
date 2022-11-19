use serde::{Deserialize, Serialize};

use super::composition::Composition;

#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    pub id: i64,
    pub name: String,
    pub compositions: Vec<Composition>,
}

#[tokio::main]
pub async fn get_artists() -> Result<Vec<Artist>, Box<dyn std::error::Error>> {
    let artists: Vec<Artist> = reqwest::get("http://127.0.0.1:8000/artists")
        .await?
        .json()
        .await?;

    Ok(artists)
}
