use std::{
    io::{BufReader, Read},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = TcpListener::bind("localhost:3000")?;

    for stream in server.incoming() {
        println!("STREAM");
        handle(BufReader::new(stream?))?;
    }

    Ok(())
}

fn handle(mut stream: BufReader<TcpStream>) -> Result<(), Box<dyn std::error::Error>> {
    let mut data = vec![0; 16];
    stream.read(data.as_mut_slice())?;

    println!("{:?}", String::from_utf8(data).unwrap());

    Ok(())
}
