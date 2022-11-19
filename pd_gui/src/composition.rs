use iced::Column;
use serde::{Deserialize, Serialize};

use super::artist::Artist;

type Sample = i32; // A sample is audio from the local-fs or ipfs (the latter with a potential pointer to a blockchain node) // TODO: Create the correct type

#[derive(Serialize, Deserialize, Debug)]
pub struct Composition {
    pub id: i64,
    pub artist: String,
    pub title: String,
    pub desc: String,
    pub samples: Vec<Sample>,
    pub collaborators: Vec<Artist>,
}

impl Default for Composition {
    fn default() -> Self {
        Self {
            id: 1,
            artist: "Myself".to_string(),
            title: "Default".to_string(),
            desc: "Default values to test with".to_string(),
            samples: vec![],
            collaborators: vec![],
        }
    }
}

impl Composition {
    #[tokio::main]
    pub async fn get_compositions(
    ) -> Result<Vec<Composition>, Box<dyn std::error::Error>> {
        let compositions: Vec<Composition> =
            reqwest::get("http://127.0.0.1:8000/compositions")
                .await?
                .json()
                .await?;

        Ok(compositions)
    }

    pub fn container<Msg>() -> Column<'static, Msg> {
        Column::new()
    }
}
