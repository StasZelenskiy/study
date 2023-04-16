use crate::http::{Request, Response, StatusCode, ParseError};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, error: &ParseError) -> Response {
        println!("Failed to parse request: {}", error);
        Response::new(StatusCode::BadRequest, None)
    }
}
