use serde::{Deserialize, Serialize};

use crate::data::data::Player;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActiveWorld {
    pub players: Vec<Player>,
    pub name: String,
}
