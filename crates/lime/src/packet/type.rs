use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PacketType {
    RAW,
    HANDSHAKE,
    LOGIN,
    MESSAGE,
    ACTION
}
