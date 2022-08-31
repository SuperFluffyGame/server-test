pub mod packet_types {
    pub const TEXT: u8 = 1u8;
}

pub enum Packet {
    Text { message: String },
}
impl Packet {
    pub fn as_bytes(&self) -> Vec<u8> {
        use Packet::*;
        match self {
            Text { message } => {
                let mut o = vec![packet_types::TEXT];
                let len = message.len() as u32;

                let len_bytes = len.to_be_bytes();

                o.extend_from_slice(&len_bytes);

                let message_bytes = message.as_bytes();
                o.extend_from_slice(message_bytes);

                o
            }
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        use packet_types::*;
        let t = *bytes.get(0).unwrap(); // !!!
        match t {
            TEXT => {
                let len = u32::from_be_bytes(*bytes.get(1..4).unwrap()); // !!!
                let string_bytes = bytes.get(5..len).unwrap(); // !!!

                let s = String::from_utf8(string_bytes).unwrap(); // !!!

                Packet::Text { message: s }
            }
        }
    }
}

pub fn hello() {
    let m = Packet::Text {
        message: "HIII".to_string(),
    };

    let bytes = m.as_bytes();

    println!("{:?}", bytes);
}
