
use ecies_ed25519::{SecretKey, PublicKey, generate_keypair, encrypt, decrypt};
use rand::{self, rngs::ThreadRng};
use anyhow::Context;

pub struct LimeE {
    sk: SecretKey,
    pk: PublicKey,
    rng: ThreadRng
}

impl LimeE {
    pub fn new() -> Self {
        let _rng = rand::thread_rng();
        let (_sk, _pk) = generate_keypair(&mut _rng.clone());
        Self {
            sk: _sk,
            pk: _pk,
            rng: _rng
        }
    }
    pub fn from(secet_key: SecretKey, public_key: PublicKey) -> Self {
        let _rng = rand::thread_rng();
        Self {
            sk: secet_key,
            pk: public_key,
            rng: _rng
        }
    }
    pub fn encrypt(&mut self, data: String) -> Vec<u8> {
        let raw = data.as_bytes();
        encrypt(&self.pk, raw, &mut self.rng)
            .context("Failed to encrypt data")
            .unwrap()
    }
    pub fn decrypt(&mut self, data: Vec<u8>) -> String {
        String::from_utf8(
            decrypt(&self.sk, &data)
                .context("Failed to decrypt the data")
                .unwrap()
        ).unwrap()
    }

}
