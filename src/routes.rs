use actix_web::{Error, Responder};
use mongodb::Database;
use serde::{Serialize, Deserialize};
use paperclip::actix::{OpenApiExt, Apiv2Schema, api_v2_operation, web::{self, Json}};

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Room {
    /// Room name
    name: String,

    /// List of power switch names
    power_switches: Vec<String>,

    /// List of thermometer names
    thermometers: Vec<String>
}

/// Get list of rooms
#[api_v2_operation]
pub async fn rooms(db: web::Data<Database>) -> Result<Json<Vec<Room>>, Error> {
    let rooms = vec![
        Room {
            name: "Гостинная".to_string(),
            power_switches: vec!["Один".to_string(), "Два".to_string()],
            thermometers: vec![]
        },
        Room {
            name: "Ванная".to_string(),
            power_switches: vec!["Один".to_string()],
            thermometers: vec!["Один".to_string()]
        },
    ];

    Ok(web::Json(rooms))
}