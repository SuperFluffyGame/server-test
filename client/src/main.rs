use common::Packet;
use std::{
    io::{BufRead, Write},
    net::TcpStream,
};

fn main() {
    let mut connection = TcpStream::connect("localhost:3000").unwrap();

    let stdin = std::io::stdin().lock();
    for line in stdin.lines() {
        let s = line.unwrap();
        let p = Packet::Text { message: s };
        connection.write_all(&p.export()).unwrap();
        connection.flush().unwrap();
    }
}
