extern crate reqwest;
extern crate serde_json;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use std::io::Cursor;
use std::*;

pub enum SimpleError {
    IoError(io::Error),
    RqwError(reqwest::Error),
    SerdeError(serde_json::Error),
}

impl From<io::Error> for SimpleError {
    fn from(error: io::Error) -> Self {
        SimpleError::IoError(error)
    }
}

impl From<reqwest::Error> for SimpleError {
    fn from(error: reqwest::Error) -> Self {
        SimpleError::RqwError(error)
    }
}

impl From<serde_json::Error> for SimpleError {
    fn from(error: serde_json::Error) -> Self {
        SimpleError::SerdeError(error)
    }
}

impl<'r> Responder<'r> for SimpleError {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .status(Status::NotAcceptable)
            .sized_body(Cursor::new(""))
            .header(ContentType::new("application", "json"))
            .ok()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
