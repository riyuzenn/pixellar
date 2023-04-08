use serde::{Serialize, Deserialize};

pub mod active;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldItem {
    background: i32,
    foreground: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct World {
    name: &'static str,
    item: Vec<WorldItem>,
} 
