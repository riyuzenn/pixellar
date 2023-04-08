use serde::{Serialize, Deserialize};

use crate::data::data::Player;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActiveWorld {
    pub players: Vec<Player>,
    pub name: &'static str
}
