# Rust Server

This project aim to create HTTP server with Rust standard library, including handling GET, POST, PUT, DELETE request, display log and response message.

## Create Rust Project

```bash
cargo init
```

## Rust Standard Library

Visit [Official website](https://doc.rust-lang.org/std/index.html) for searching Rust standard library. These below are the standard libraries for creating HTTP server.

- std::net::TcpListener - create http server
- std::io::Read; - read buffer, input, output
- std::str; - convert string utf
- std::fmt:Display - display message
- std::convert::TryFrom - throw error when the value is not existed
- std::fmt::Debug - implement debug automatically for printing object on command line
- std::convert::From - doesn't throw error when it is not existed
- std::str::FromStr - convert string to enum or others.

## Importing module

```rust
// mod.rs
pub mod <FILE_NAME>; // import file

pub use <FILE_NAME>::<STRUCT_NAME> // use struct

// other_file.rs import module
use crate::<FILE_NAME>::<STRUCT_NAME>

// import standard module
use std::<NAME>
```

## Send request to Rust server with curl

```bash
# GET request
curl localhost:8000

# Server log
GET /employees HTTP/1.1
Host: localhost:8000
User-Agent: curl/7.78.0
Accept: */*

Request {
    method: GET,
    path: "/employees",
    query_string: None,
}
```

```bash
# POST request
curl localhost:8000 -XPOST

# Server log
GET /employees HTTP/1.1
Host: localhost:8000
User-Agent: curl/7.78.0
Accept: */*

Request {
    method: POST,
    path: "/employees",
    query_string: None,
}
```

```bash
# GET request with params
curl "localhost:8000/employees?name=carlos&salary=30000"

# Server log
GET /employees?name=carlos&salary=30000 HTTP/1.1
Host: localhost:8000
User-Agent: curl/7.78.0
Accept: */*

Request {
    method: GET,
    path: "/employees",
    query_string: Some(
        QueryString {
            data: {
                "salary": "30000",
                "name": "carlos",
            },
        },
    ),
}
```
