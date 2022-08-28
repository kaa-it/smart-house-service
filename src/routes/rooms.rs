use mongodb::Database;
use serde::{Serialize, Deserialize};
use paperclip::actix::{Apiv2Schema, api_v2_operation, web::{self, Json}, CreatedJson};
use crate::error::ApplicationError;
use crate::persistence;
use crate::persistence::rooms::{NewRoomEntity, RoomEntity};

pub fn rooms_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/rooms")
            .route(web::get().to(rooms))
            .route(web::post().to(add_room)),
    );
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Room {
    /// Room name
    name: String,

    /// List of power switch names
    power_switches: Vec<String>,

    /// List of thermometer names
    thermometers: Vec<String>
}

impl From<&RoomEntity> for Room {
    fn from(room: &RoomEntity) -> Self {
        Self {
            name: room.name.clone(),
            power_switches: room.power_switches.clone(),
            thermometers: room.thermometers.clone(),
        }
    }
}

/// Get list of rooms
#[api_v2_operation]
pub async fn rooms(db: web::Data<Database>) -> Result<Json<Vec<Room>>, ApplicationError> {
    let rooms = match persistence::rooms::rooms(&db).await {
        Err(e) => {
            return Err(ApplicationError::InternalServerError {message: e.to_string()});
        }
        Ok(r) => r,
    };

    Ok(web::Json(rooms.iter().map(Room::from).collect()))
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct AddRoomRequest {
    /// Room name
    name: String,
}

/// Add new room to smart house
#[api_v2_operation]
pub async fn add_room(db: web::Data<Database>, room: Json<AddRoomRequest>) -> Result<CreatedJson<Room>, ApplicationError> {
    let new_room = NewRoomEntity { name: room.name.clone() };

    let room = match persistence::rooms::add_room(&db, &new_room).await {
        Err(e) => {
            return match e.downcast_ref::<crate::persistence::error::Error>() {
                Some(crate::persistence::error::Error::AlreadyExistsError) => {
                    Err(ApplicationError::RoomAlreadyExistsError { name: room.name.clone() })
                }
                Some(_) => unreachable!(),
                None => Err(ApplicationError::InternalServerError { message: e.to_string() }),
            }
        }
        Ok(room) => room,
    };

    Ok(CreatedJson(Room::from(&room)))
}