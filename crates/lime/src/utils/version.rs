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

use anyhow::{
    Result,
    Context
};
use colored::Colorize;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl Version {
    pub fn new(m: u8, mi: u8, p: u8) -> Version {
        Version {
            major: m,
            minor: mi,
            patch: p,
        }
    }

    pub fn from(s: &str) -> Result<Version> {
        let v: Vec<u8> = s.split(".")
            .map(|x| x.parse::<u8>()
            .context(format!("{}", "Cannot convert str to u8".red())).unwrap())
            .collect();

        Ok(Version {
            major: v[0],
            minor: v[1],
            patch: v[2],
        })
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    pub fn compare(&self, v: Version) -> bool {
        let a = self.collect();
        let b = v.collect();

        let m = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        if m == 3 {
            return true
        } else {
            return false
        }
    }

    pub fn collect(&self) -> Vec<u8> {
        vec![self.major, self.minor, self.patch]
    }
}

