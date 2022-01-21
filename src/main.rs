use std::io::Write;
use std::net::{TcpListener, TcpStream, Shutdown};

fn handle_client(mut stream: TcpStream) {
    println!("O servidor recebeu uma conexão");

    let data = b"<h1>oi</h1>";

    let bytes_escritos = stream.write_all(data);
    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}