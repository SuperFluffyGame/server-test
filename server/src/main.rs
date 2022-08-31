use common::Packet;
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    thread,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = TcpListener::bind("localhost:3000")?;

    for stream in server.incoming() {
        println!("STREAM");
        thread::spawn(|| handle(stream.unwrap()).unwrap());
    }

    Ok(())
}

fn handle(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut data = [0; 16 * 1024];

    loop {
        stream.read(data.as_mut_slice())?;
        let packet = Packet::parse(&data);
        println!("{:?}", packet);
    }
}
