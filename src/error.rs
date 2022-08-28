use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use paperclip::actix::api_v2_errors;
use thiserror::Error;

/// Describes errors of the app
#[derive(Error, Debug)]
#[api_v2_errors(code = 409, code = 500, code = 404)]
pub enum ApplicationError {
    /// Describes error in case of room already exists
    #[error(r#"Room with name '{}' already exists in smart house"#, name)]
    RoomAlreadyExists { name: String },

    /// Describes error in case of power switch already exists in room
    #[error(
        r#"Power switch with name '{}' already exists in room with name '{}'"#,
        name,
        room_name
    )]
    PowerSwitchAlreadyExists { name: String, room_name: String },

    /// Describes error in case of thermometer already exists in room
    #[error(
        r#"Thermometer with name '{}' already exists in room with name '{}'"#,
        name,
        room_name
    )]
    ThermometerAlreadyExists { name: String, room_name: String },

    /// Describes error in case of room is not found
    #[error(r#"Room with name '{}' not found"#, name)]
    RoomNotFound { name: String },

    /// Describes error in case of power switch is not found
    #[error(
        r#"Power switch with name '{}' not found in room with name '{}"#,
        name,
        room_name
    )]
    PowerSwitchNotFound { name: String, room_name: String },

    /// Describes error in case of thermometer is not found
    #[error(
        r#"Thermometer with name '{}' not found in room with name '{}"#,
        name,
        room_name
    )]
    ThermometerNotFound { name: String, room_name: String },

    /// Internal Server Error
    #[error("Internal server error: {}", message)]
    InternalServer { message: String },
}

impl error::ResponseError for ApplicationError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(format!(
                r#"
            {{
               "error": "{}"
            }}
            "#,
                self
            ))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApplicationError::InternalServer { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ApplicationError::RoomAlreadyExists { .. } => StatusCode::CONFLICT,
            ApplicationError::PowerSwitchAlreadyExists { .. } => StatusCode::CONFLICT,
            ApplicationError::ThermometerAlreadyExists { .. } => StatusCode::CONFLICT,
            ApplicationError::RoomNotFound { .. } => StatusCode::NOT_FOUND,
            ApplicationError::PowerSwitchNotFound { .. } => StatusCode::NOT_FOUND,
            ApplicationError::ThermometerNotFound { .. } => StatusCode::NOT_FOUND,
        }
    }
}
