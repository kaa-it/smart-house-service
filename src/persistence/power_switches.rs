use bson::{doc, Document};
use mongodb::Database;
use crate::persistence::utils::check_already_exists;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPowerSwitchEntity {
    pub name: String,
    pub room_name: String,
    pub description: String,
    pub power_consumption: f64
}

pub async fn add_power_switch(db: &Database, new_power_switch: &NewPowerSwitchEntity) -> anyhow::Result<()> {
    let docs = db.collection::<Document>("power_switches");

    let power_switch = doc! {
        "name": &new_power_switch.name,
        "room_name": &new_power_switch.room_name,
        "description": &new_power_switch.description,
        "power_consumption": new_power_switch.power_consumption
    };

    if let Err(e) = docs.insert_one(power_switch, None).await {
        return Err(check_already_exists(e));
    };

    let rooms = db.collection::<Document>("rooms");

    let filter = doc! {
        "name": &new_power_switch.room_name
    };

    let update = doc! {
        "$push": {
            "power_switches": &new_power_switch.name
        }
    };

    rooms.update_one(filter.clone(), update, None).await?;

    Ok(())
}