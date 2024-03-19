use openssl::ssl::{SslConnector, SslMethod};
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

fn main() {
    // let host = "pokeapi.co";
    let host = "www.google.com";
    let port = 443;
    let host_and_port = (host, port);

    println!("Connecting to {}", host);

    let ip = match host_and_port.to_socket_addrs() {
        Ok(mut addresses) => match addresses.next() {
            Some(address) => address,
            None => {
                eprintln!("No address found for {}", host);
                return;
            }
        },
        Err(e) => {
            eprintln!("Failed to lookup address for {}: {}", host, e);
            return;
        }
    };

    println!("Connecting to {}", ip);

    let stream = match TcpStream::connect(ip) {
        Ok(stream) => stream,
        Err(er) => {
            eprintln!("Failed to connect to {}: {}", ip, er);
            return;
        }
    };

    println!("Got a stream");

    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
    let mut ssl_stream = match connector.connect(host, stream) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Failed to connect to {}: {}", host, e);
            return;
        }
    };

    println!("ssl_stream: {:?}", ssl_stream);

    let request = format!(
        "GET / HTTP/1.1\r\nHost: {}\r\nAccept: */*\r\nConnection: close\r\n\r\n",
        host
    );

    if let Err(e) = ssl_stream.write_all(request.as_bytes()) {
        eprintln!("Failed to write to stream: {}", e);
    }

    println!("request!");

    let mut response = String::new();
    if let Err(e) = ssl_stream.read_to_string(&mut response) {
        eprintln!("Failed to read from stream: {}", e);
    }

    println!("response - {}", response);
}
