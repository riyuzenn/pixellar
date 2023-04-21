use serde::{Deserialize, Serialize};

use crate::data::Item;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerInventory {
    pub items: Vec<Item>,
    pub size: i32,
}
