use actix_web::{Error, Responder, web};
use actix_web::web::Json;
use mongodb::Database;
use serde::{Serialize, Deserialize};
use paperclip::actix::{OpenApiExt, Apiv2Schema, api_v2_operation};

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Room {
    name: String,
    power_switches: Vec<String>,
    thermometers: Vec<String>
}

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