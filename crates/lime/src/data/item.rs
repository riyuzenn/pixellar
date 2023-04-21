use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ItemType {
    WEARABLE,
    BLOCK,
    DECOR,
    WALL,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ItemRarity {
    NATURAL,
    BASIC,
    RARE,
    ULTRARARE,
    LEGENDARY,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub r#type: ItemType,
    pub rarity: ItemRarity,
}
