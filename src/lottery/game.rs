use std::{fmt::Display, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    pub id: String,
    pub description: String,
    pub draw_days: Vec<String>,
    pub pools: Vec<Pool>,
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pool {
    pub count: u8,
    pub range: [u8; 2],
}

impl Game {
    fn config_file_path() -> Option<PathBuf> {
        let candidates = vec![
            PathBuf::from("./games.json"),
            dirs::config_dir()
                .map(|d| d.join("lottery/games.json"))
                .unwrap_or_default(),
            PathBuf::from("/etc/lottery/games.json"),
        ];

        for p in candidates {
            if p.exists() {
                return Some(p);
            }
        }
        None
    }

    pub fn load() -> Vec<Game> {
        let games_config = Self::config_file_path();
        match games_config {
            Some(path) => {
                let data = std::fs::read_to_string(path)
                    .expect("Failed to read games.json from config path");
                serde_json::from_str(&data).expect("Failed to parse games.json from config path")
            }
            None => {
                let data = include_str!("../../data/games.json");
                serde_json::from_str(&data).expect("Failed to parse games.json from embedded data")
            }
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.id)
    }
}
