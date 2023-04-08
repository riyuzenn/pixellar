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

use std::path::{Path, PathBuf};

use anyhow::{
    Result,
    Context
};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

pub use crate::utils::version::Version;
pub use crate::utils::bunny::say;

pub mod version;
pub mod bunny;

pub fn host_to_vec(host: &str) -> Result<Vec<u8>> {
    let mut h = Vec::new();
    let p = host.split(".")
        .collect::<Vec<&str>>();
    
    for x in p {
        h.push(
            x.parse::<u8>()
                .context("Failed to convert to u8. Perhaps your parameter is not formatted properly?")?
        );
    }

    Ok(h)
}

pub fn get_db_path() -> PathBuf {
    let p = Path::new("./db");
    if !p.exists() {
        std::fs::create_dir(p).unwrap();
    }
    p.to_owned()
}

pub fn load_db(name: &str) -> Result<PickleDb, anyhow::Error> {
    let db_env = get_db_path();
    let db_path = db_env.join(format!("{}.db", name));
    if db_path.exists() {
        
        Ok(
            PickleDb::load_json(
                db_path, 
                PickleDbDumpPolicy::DumpUponRequest
            ).context("Failed to open database path.")?
        )
    } else {
        Ok(
            PickleDb::new(
                db_path,
                PickleDbDumpPolicy::DumpUponRequest,
                SerializationMethod::Json
            )
        )
    }
}

