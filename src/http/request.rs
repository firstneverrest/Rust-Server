use crate::http::Method;
use crate::http::QueryString;
use crate::http::Error;
use crate::http::Result;
use std::convert::TryFrom;
use std::str;

#[derive(std::fmt::Debug)] // implement debug automatically

pub struct Request {
    method: Method,
    path: String,
    query_string: Option<QueryString>, // Option has two variants: Some(T) and None
}

impl Request {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = Error;
    fn try_from(buf: &[u8]) -> Result<Self> {
        let request = str::from_utf8(&buf)?;
        println!("{}", request);

        // GET /employees?name=carlos&salary=30000 HTTP/1.1
        let mut request = request.split_whitespace();
        let method = request.next().ok_or(Error::InvalidRequest)?; // throw error
        let mut path = request.next().ok_or(Error::InvalidRequest)?;
        let protocol = request.next().ok_or(Error::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(Error::InvalidProtocol);
        }

        // convert string to enum
        let method: Method = method.parse()?;

        // find query params
        let query_string = match path.find('?') {
            Some(i) => {
                let query_string = &path[i+1..];
                path = &path[..i];
                Some(QueryString::from(query_string))
            }
            None => None
        };
        

        Ok(Self {
            method: method,
            path: path.to_string(),
            query_string: query_string,
        })
    }
}