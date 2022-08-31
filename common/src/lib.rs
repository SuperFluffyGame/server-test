use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Packet {
    Null,
    Text { message: String },
}
impl Packet {
    pub fn parse(bytes: &[u8]) -> Self {
        deserialize::<Self>(bytes).unwrap()
    }
    pub fn export(&self) -> Vec<u8> {
        serialize(self).unwrap()
    }
}
