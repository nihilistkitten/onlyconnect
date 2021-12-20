use rocket::{
    http::{ContentType, Status},
    Response,
};

use std::io::Cursor;

/// A custom error type.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error occured with the database.
    #[error("Database Error: {0}")]
    DatabaseErr(#[from] diesel::result::Error),
}

impl<'r> rocket::response::Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let status = match self {
            Error::DatabaseErr(_) => Status::InternalServerError,
        };
        let body = self.to_string();
        Ok(Response::build()
            .status(status)
            .header(ContentType::Plain)
            .sized_body(body.len(), Cursor::new(body))
            .finalize())
    }
}
