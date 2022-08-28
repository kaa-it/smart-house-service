use actix_web::{error, HttpResponse};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use thiserror::Error;
use paperclip::actix::api_v2_errors;

/// Describes errors of the app
#[derive(Error, Debug)]
#[api_v2_errors(
    code=409,
    code=500,
)]
pub enum ApplicationError {
    /// Describes error in case of room already exists
    #[error(r#"Room with name "{}" already exists in smart house"#, name)]
    RoomAlreadyExistsError {
        name: String,
    },

    /// Describes error in case of power switch already exists in room
    #[error(r#"Power switch with name "{}" already exists in room with name "{}"#, name, room_name)]
    PowerSwitchAlreadyExistsError {
        name: String,
        room_name: String,
    },

    /// Internal Server Error
    #[error("Internal server error: {}", message)]
    InternalServerError {
        message: String,
    }
}

impl error::ResponseError for ApplicationError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(format!(r#"
            {{
               "error": {}
            }}
            "#, self.to_string()))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApplicationError::InternalServerError{..} => StatusCode::INTERNAL_SERVER_ERROR,
            ApplicationError::RoomAlreadyExistsError{..} => StatusCode::CONFLICT,
            ApplicationError::PowerSwitchAlreadyExistsError{..} => StatusCode::CONFLICT
        }
    }
}