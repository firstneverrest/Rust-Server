pub mod server;
pub mod error;
pub mod method;
pub mod query_string;
pub mod request;
pub mod status;
pub mod response;

pub use error::Error;
pub use method::Method;
pub use server::Server;
pub use query_string::QueryString;
pub use request::Request;
pub use status::HttpStatus;
pub use response::Response;

pub type Result<T> = std::result::Result<T, Error>;