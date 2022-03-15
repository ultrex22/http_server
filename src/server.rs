use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Server listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            let res = listener.accept();
            match res {
                Ok((mut stream, _addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(nb) => {
                            println!(
                                "Received a request: {}, Number of bytes: {}",
                                String::from_utf8_lossy(&buffer),
                                nb
                            );
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(e) => println!("Received an error, {}", e),
                            }
                        }
                        Err(e) => println!("Error: {}", e),
                    }
                }
                Err(e) => {
                    println!("Error: {}", e)
                }
            }
        }
    }
}
