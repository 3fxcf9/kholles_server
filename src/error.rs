use rocket::response::Responder;
use rocket_dyn_templates::{context, Template};
use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ErrorType {
    ServerError,
    ClientError,
    RuntimeError,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ClientError => "Client error",
                Self::ServerError => "Server error",
                Self::RuntimeError => "Runtime error",
            }
        )
    }
}

#[derive(Debug)]
pub struct CustomError {
    error_type: ErrorType,
    message: String,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.error_type, self.message)
    }
}

impl Error for CustomError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<io::Error> for CustomError {
    fn from(error: io::Error) -> Self {
        CustomError {
            error_type: ErrorType::RuntimeError,
            message: error.to_string(),
        }
    }
}

impl CustomError {
    pub fn new(error_type: ErrorType, message: String) -> Self {
        CustomError {
            error_type,
            message,
        }
    }
    pub fn as_template(&self) -> Template {
        Template::render(
            "error",
            context! {
                title: self.error_type.to_string(),
                text: self.to_string(),
            },
        )
    }
}

impl<'r> Responder<'r, 'static> for CustomError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        self.as_template().respond_to(request)
    }
}
