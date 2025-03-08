use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{Request, Response as RocketResponse};
use std::fmt;
use std::io::Cursor;

#[derive(Debug, Serialize)]
pub struct WebhookError {
    pub status: Status,
    pub message: String,
}

impl WebhookError {
    pub fn new(status: Status, message: String) -> Self {
        WebhookError { status, message }
    }
}

impl fmt::Display for WebhookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.status, self.message)
    }
}

impl WebhookError {
    pub fn into_response(self) -> RocketResponse<'static> {
        let body = Json(&self);
        Response::build()
            .status(self.status)
            .sized_body(body.to_string().len(), Cursor::new(body.to_string()))
            .finalize()
    }
}

impl<'r> Responder<'r, 'static> for WebhookError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        Ok(self.into_response())
    }
}
