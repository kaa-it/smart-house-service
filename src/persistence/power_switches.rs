use crate::persistence::error::Error::NotFoundError;
use crate::persistence::utils::check_already_exists;
use bson::{doc, Document};
use mongodb::Database;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPowerSwitchEntity {
    pub name: String,
    pub room_name: String,
    pub description: String,
    pub power_consumption: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemovePowerSwitchEntity {
    pub name: String,
    pub room_name: String,
}

pub async fn add_power_switch(
    db: &Database,
    new_power_switch: &NewPowerSwitchEntity,
) -> anyhow::Result<()> {
    let rooms = db.collection::<Document>("rooms");

    let filter = doc! {
        "name": &new_power_switch.room_name
    };

    let room = rooms.find_one(filter, None).await?;

    if room.is_none() {
        return Err(NotFoundError.into());
    }

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

pub async fn remove_power_switch(
    db: &Database,
    remove_power_switch: &RemovePowerSwitchEntity,
) -> anyhow::Result<()> {
    let power_switches = db.collection::<Document>("power_switches");

    let filter = doc! {
        "name": &remove_power_switch.name,
        "room_name": &remove_power_switch.room_name
    };

    let ps = power_switches.find_one(filter.clone(), None).await?;

    if ps.is_none() {
        return Err(NotFoundError.into());
    }

    power_switches.delete_one(filter, None).await?;

    let rooms = db.collection::<Document>("rooms");

    let query = doc! {
        "name": &remove_power_switch.room_name
    };

    let update = doc! {
        "$pull": {
            "power_switches": &remove_power_switch.name
        }
    };

    rooms.update_one(query, update, None).await?;

    Ok(())
}
