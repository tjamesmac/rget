use openssl::ssl::{SslConnector, SslMethod};
use std::env;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use url::Url;

fn main() {
    let args: Vec<String> = env::args().collect();

    let verb = &args[1];

    let before_url = Url::parse(&args[2]);

    let url = match before_url {
        Ok(url) => url,
        Err(e) => {
            panic!("Error parsing URL: {}", e);
        }
    };

    let port = match url.scheme() {
        "https" => 443,
        "http" => 80,
        _ => panic!("Unknown scheme: {}", url.scheme()),
    };

    let host = url.host_str().unwrap();

    let host_and_port = (host, port);

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

    let request = format!(
        "{} {} HTTP/1.1\r\nHost: {}\r\nAccept: */*\r\nConnection: close\r\n\r\n",
        verb.to_uppercase(),
        url.path(),
        host
    );

    if let Err(e) = ssl_stream.write_all(request.as_bytes()) {
        eprintln!("Failed to write to stream: {}", e);
    }

    let mut response = String::new();

    if let Err(e) = ssl_stream.read_to_string(&mut response) {
        eprintln!("Failed to read from stream: {}", e);
    }

    println!("response - {}", response);
}
