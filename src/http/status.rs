use std::fmt::Display;

pub enum HttpStatus {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        let message = match self {
            Self::Ok => "200 OK",
            Self::BadRequest => "400 Bad Request",
            Self::NotFound => "404 Not Found",
        };
        write!(f, "{}", message)
    }
}