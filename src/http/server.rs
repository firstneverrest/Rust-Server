use std::net::TcpListener;
use std::io::Read;
use crate::http::Result;
use crate::http::Request;
use crate::http::Method;
use crate::http::Response;
use crate::http::HttpStatus;

pub struct Server {
    addr: String,
}

impl Server {
    // constructor
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    // pub fn run(&self) -> Result<(), std::io::Error> {
    //     println!("Listening on http://{}", self.addr);

    //     let listener = TcpListener::bind(&self.addr)?;

    //     Ok(())
    // }

    pub fn run(&self) -> Result<()> {
        println!("Listening on http://{}", self.addr);

        // ? is used to throw error
        let listener = TcpListener::bind(&self.addr)?;

        // accept connections until server is closed
        for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buf: [u8; 1024] = [0; 1024]; // buffer
            stream.read(&mut buf)?;

            let request = Request::try_from(&buf[..])?; // must send slice of utf8
            println!("{:#?}", request); // {:#?} print object and format it

            let response = match request.method() {
                Method::GET => match request.path().as_str() {
                    "/" => Response::new(HttpStatus::Ok, Some("home".to_string())),
                    "/employees" => Response::new(HttpStatus::Ok, Some("employees".to_string())),
                    _ => Response::new(HttpStatus::NotFound, None),
                },
                
                _ => Response::new(HttpStatus::NotFound, None),
            };

            response.send(&mut stream)?;
        }
        
        Ok(())
    }
}