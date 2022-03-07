use std::net::TcpListener;
use std::io::Read;
use std::str;

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

    pub fn run(&self) -> std::io::Result<()> {
        println!("Listening on http://{}", self.addr);

        // ? is used to throw error
        let listener = TcpListener::bind(&self.addr)?;

        // accept connections until server is closed
        for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buf: [u8; 1024] = [0; 1024]; // buffer
            stream.read(&mut buf)?;
            
            if let Ok(request) = str::from_utf8(&buf) {
                println!("Request: {}", request);
            }
            // println!("Connection established!");
        }
        
        Ok(())
    }
}