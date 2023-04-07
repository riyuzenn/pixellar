/*
 * Copyright (c) 2023 riyuzenn <riyuzenn@gmail.com>
 * See the license file for more info
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
use serde::{Serialize, Deserialize};
use enet::PacketMode;

use anyhow::Result;
use crate::utils::Version;
use crate::packet::r#type::PacketType;

#[derive(Serialize, Deserialize, Clone)]
pub struct PacketData {
    pub data: Vec<u8>,
    pub version: Version,
    pub r#type: PacketType,
}

impl PacketData {
    pub fn new(d: Vec<u8>, v: Version, t: PacketType) -> Self {
        PacketData {
            data: d,
            version: v,
            r#type: t,
        }
    }

    pub fn build(&self) -> Result<Vec<u8>, String> { 
        match serde_json::to_string(&self) {
            Ok(data) => Ok(data.into_bytes()),
            Err(err) => Err(err.to_string())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PacketRawData {
    data: Vec<u8>,
    version: Version,
}


impl PacketRawData {
    pub fn new(d: Vec<u8>, v: Version) -> Self {
        PacketRawData { data: d, version: v }
    }

    /// Build the packet 
    pub fn build(&self) {

    }
}

pub struct Packet {
    pkt: enet::Packet,
}

impl Packet {
    pub fn new(d: Vec<u8>, packet_mode: PacketMode) -> Self {
        
        let p = enet::Packet::new(d, packet_mode).unwrap();
        Packet {
            pkt: p
        }
    }

    pub fn build(&self) {

    }

    pub fn send(&self, peer: enet::Peer<()>) {
        
    }
}
