use crate::error::ApplicationError;
use crate::persistence::thermometers;
use crate::persistence::thermometers::{NewThermometerEntity, RemoveThermometerEntity};
use actix_web::HttpResponse;
use mongodb::Database;
use paperclip::actix::{
    api_v2_operation,
    web::{self, Json},
    Apiv2Schema, CreatedJson,
};
use serde::{Deserialize, Serialize};

pub fn thermometers_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/thermometers")
            .route(web::post().to(add_thermometer))
            .route(web::delete().to(remove_thermometer)),
    );
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct AddThermometerRequest {
    /// Room name
    room_name: String,

    /// Power switch name
    name: String,

    /// Description
    description: String,

    /// Temperature
    temperature: f64,
}

/// Add new thermometer to room
#[api_v2_operation]
pub async fn add_thermometer(
    db: web::Data<Database>,
    thermometer: Json<AddThermometerRequest>,
) -> Result<CreatedJson<()>, ApplicationError> {
    let new_thermometer = NewThermometerEntity {
        name: thermometer.name.clone(),
        room_name: thermometer.room_name.clone(),
        description: thermometer.description.clone(),
        temperature: thermometer.temperature,
    };

    if let Err(e) = thermometers::add_thermometer(&db, &new_thermometer).await {
        return match e.downcast_ref::<crate::persistence::error::Error>() {
            Some(crate::persistence::error::Error::AlreadyExistsError) => {
                Err(ApplicationError::ThermometerAlreadyExists {
                    name: new_thermometer.name.clone(),
                    room_name: new_thermometer.room_name.clone(),
                })
            }
            Some(crate::persistence::error::Error::NotFoundError) => {
                Err(ApplicationError::RoomNotFound {
                    name: new_thermometer.room_name.clone(),
                })
            }
            None => Err(ApplicationError::InternalServer {
                message: e.to_string(),
            }),
        };
    };

    Ok(CreatedJson(()))
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct RemoveThermometerRequest {
    /// Room name
    room_name: String,

    /// Power switch name
    name: String,
}

/// Remove thermometer from room
#[api_v2_operation]
pub async fn remove_thermometer(
    db: web::Data<Database>,
    thermometer: Json<RemoveThermometerRequest>,
) -> Result<HttpResponse, ApplicationError> {
    let remove_thermometer = RemoveThermometerEntity {
        name: thermometer.name.clone(),
        room_name: thermometer.room_name.clone(),
    };

    if let Err(e) = thermometers::remove_thermometer(&db, &remove_thermometer).await {
        return match e.downcast_ref::<crate::persistence::error::Error>() {
            Some(crate::persistence::error::Error::NotFoundError) => {
                Err(ApplicationError::ThermometerNotFound {
                    name: remove_thermometer.name.clone(),
                    room_name: remove_thermometer.room_name.clone(),
                })
            }
            Some(_) => unreachable!(),
            None => Err(ApplicationError::InternalServer {
                message: e.to_string(),
            }),
        };
    };

    Ok(HttpResponse::Ok().into())
}
