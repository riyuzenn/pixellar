use std::time::Duration;
use serde::{Serialize, Deserialize};

use enet::PeerID;
use super::inv::PlayerInventory;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub uid: String,
    pub connect_id: PeerID,
    pub name: String,
    pub inventory: PlayerInventory,
    pub password: String,
    pub banned: bool,
    pub muted: bool,
    pub created_at: Duration,
    pub public_key: [u8; 32],
    pub secret_key: [u8; 32]
}
