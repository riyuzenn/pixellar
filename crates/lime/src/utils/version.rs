
use anyhow::{
    Result,
    Context
};
use colored::Colorize;

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

