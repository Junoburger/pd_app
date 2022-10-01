use iced::Column;
use serde::{Deserialize, Serialize};

use crate::artist::Artist;

#[derive(Serialize, Deserialize, Debug)]
pub struct Composition {
    pub id: i64,
    pub artist: String,
    pub title: String,
    pub desc: String,
    pub samples: Vec<i64>,
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
        Column::new().spacing(20)
    }
}
