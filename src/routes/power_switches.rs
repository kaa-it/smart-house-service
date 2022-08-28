use crate::error::ApplicationError;
use crate::persistence::power_switches::NewPowerSwitchEntity;
use mongodb::Database;
use paperclip::actix::{api_v2_operation, web::{self, Json}, Apiv2Schema, CreatedJson};
use crate::persistence::power_switches;
use serde::{Serialize, Deserialize};

pub fn power_switches_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/power_switches").route(web::post().to(add_power_switch)));
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct AddPowerSwitchRequest {
    /// Room name
    room_name: String,

    /// Power switch name
    name: String,

    /// Description
    description: String,

    /// Power consumption
    power_consumption: f64,
}

/// Add new power switch to room
#[api_v2_operation]
pub async fn add_power_switch(
    db: web::Data<Database>,
    power_switch: Json<AddPowerSwitchRequest>,
) -> Result<CreatedJson<()>, ApplicationError> {
    let new_power_switch = NewPowerSwitchEntity {
        name: power_switch.name.clone(),
        room_name: power_switch.room_name.clone(),
        description: power_switch.description.clone(),
        power_consumption: power_switch.power_consumption,
    };

    if let Err(e) = power_switches::add_power_switch(&db, &new_power_switch).await {
        return match e.downcast_ref::<crate::persistence::error::Error>() {
            Some(crate::persistence::error::Error::AlreadyExistsError) => {
                Err(ApplicationError::PowerSwitchAlreadyExistsError {
                    name: new_power_switch.name.clone(),
                    room_name: new_power_switch.room_name.clone(),
                })
            }
            None => Err(ApplicationError::InternalServerError {
                message: e.to_string(),
            }),
        }
    };

    Ok(CreatedJson(()))
}
