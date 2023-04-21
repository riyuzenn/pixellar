use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::inv::PlayerInventory;
use enet::PeerID;

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
    pub secret_key: [u8; 32],
}
