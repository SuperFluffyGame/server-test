use std::{io::Write, net::TcpStream};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = TcpStream::connect("localhost:3000")?;

    let s = b"Hello World!";
    connection.write(s)?;

    common::hello();

    Ok(())
}
