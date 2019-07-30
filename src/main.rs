use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use rusthello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {:?}", &buffer[..]);
    let handshake: [u8; 54] = [254, 1, 250, 0, 11, 0, 77, 0, 67, 0, 124, 0, 80, 0, 105, 0, 110, 0, 103, 0, 72, 0, 111, 0, 115, 0, 116, 0, 25, 127, 0, 9, 0, 49, 0, 50, 0, 55, 0, 46, 0, 48, 0, 46, 0, 48, 0, 46, 0, 49, 0, 0, 30, 198];

    if buffer.starts_with(&handshake) || true {
        let response = "
{
    \"version\": {
        \"name\": \"1.8.7\",
        \"protocol\": 47
    },
    \"players\": {
        \"max\": 100,
        \"online\": 42,
        \"sample\": []
    },
    \"description\": {
            \"text\": \"hello Minecraft.rs (#§6Rust§r)\"
    }
}";
        let mut buffer = Vec::new();
        write_varint(&(0 as i32), &mut buffer).unwrap(); // id
        write_varint(&(response.as_bytes().len() as i32), &mut buffer).unwrap(); // length
        buffer.write_all(response.as_bytes()).unwrap(); // response

        write_varint(&(buffer.len() as i32), &mut stream).unwrap();
        stream.write_all(&buffer).unwrap();
        stream.flush().unwrap();
    }
}

// Copied from:
/// Write a single i32 to the Writer, as a varint
pub fn write_varint<W: Write>(val: &i32, writer: &mut W) -> std::io::Result<()> {
    let msb: u8 = 0b10000000;
    let mask: i32 = 0b01111111;

    let mut val = *val;
    for _ in 0..5 {
        let tmp = (val & mask) as u8;
        val &= !mask;
        val = val.rotate_right(7);

        if val != 0 {
            writer.write_all(&[tmp | msb])?;
        } else {
            writer.write_all(&[tmp])?;
            return Ok(());
        }
    }

    panic!("Internal error in write_varint, loop ended");
}