use sha2::{Digest, Sha256};

pub struct Event {
    id: [u8; 32],
    pub_key: [u8; 32],
    created_at: u32,
    kind: u8,
    tag: Vec<String>,
    content: String,
    sig: [u8; 64],
}

impl Event {
    pub fn generate_id(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update("0,".as_bytes());
        hasher.update(&self.pub_key);
        hasher.update(",".as_bytes());
        hasher.update(&self.created_at.to_be_bytes());
        hasher.update(",".as_bytes());
        hasher.update(self.kind.to_be_bytes());
        hasher.update(",".as_bytes());
        hasher.update(self.content.as_bytes());
        let id = hasher.finalize().to_vec();
        self.id = match id.try_into() {
            Ok(id) => id,
            Err(o) => panic!("Expected a Vec of length {} but it was {}", 4, o.len()),
        };
    }
}
