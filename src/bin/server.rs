use comm::data::TestStruct;
use std::error::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() -> Result<(), Box<dyn Error>> {
    let test = TestStruct {
        name: String::from("mocos"),
        line: vec![1, 2, 3],
    };
    let listener = TcpListener::bind("0.0.0.0:1234")?;
    for stream in listener.incoming() {
        println!("Client connected!");
        let encoded: Vec<u8> = bincode::serialize(&test).unwrap();
        stream?.write(&encoded)?;
    }
    Ok(())
}
