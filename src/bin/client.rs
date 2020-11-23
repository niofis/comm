use comm::data::TestStruct;
use std::error::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() -> Result<(), Box<dyn Error>> {
    let mut server = TcpStream::connect("127.0.0.1:1234")?;
    println!("Connected to server!");
    let mut buffer = Vec::new();
    server.read_to_end(&mut buffer)?;
    let test: TestStruct = bincode::deserialize(&buffer[..]).unwrap();
    println!("{:?}", test);
    Ok(())
}
