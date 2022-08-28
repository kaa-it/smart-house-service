use crate::persistence::error::Error::NotFoundError;
use crate::persistence::utils::check_already_exists;
use bson::{doc, Document};
use mongodb::Database;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewThermometerEntity {
    pub name: String,
    pub room_name: String,
    pub description: String,
    pub temperature: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveThermometerEntity {
    pub name: String,
    pub room_name: String,
}

pub async fn add_thermometer(
    db: &Database,
    new_thermometer: &NewThermometerEntity,
) -> anyhow::Result<()> {
    let rooms = db.collection::<Document>("rooms");

    let filter = doc! {
        "name": &new_thermometer.room_name
    };

    let room = rooms.find_one(filter, None).await?;

    if room.is_none() {
        return Err(NotFoundError.into());
    }

    let docs = db.collection::<Document>("thermometers");

    let thermometer = doc! {
        "name": &new_thermometer.name,
        "room_name": &new_thermometer.room_name,
        "description": &new_thermometer.description,
        "temperature": new_thermometer.temperature
    };

    if let Err(e) = docs.insert_one(thermometer, None).await {
        return Err(check_already_exists(e));
    };

    let rooms = db.collection::<Document>("rooms");

    let filter = doc! {
        "name": &new_thermometer.room_name
    };

    let update = doc! {
        "$push": {
            "thermometers": &new_thermometer.name
        }
    };

    rooms.update_one(filter.clone(), update, None).await?;

    Ok(())
}

pub async fn remove_thermometer(
    db: &Database,
    remove_thermometer: &RemoveThermometerEntity,
) -> anyhow::Result<()> {
    let thermometers = db.collection::<Document>("thermometers");

    let filter = doc! {
        "name": &remove_thermometer.name,
        "room_name": &remove_thermometer.room_name
    };

    let ps = thermometers.find_one(filter.clone(), None).await?;

    if ps.is_none() {
        return Err(NotFoundError.into());
    }

    thermometers.delete_one(filter, None).await?;

    let rooms = db.collection::<Document>("rooms");

    let query = doc! {
        "name": &remove_thermometer.room_name
    };

    let update = doc! {
        "$pull": {
            "thermometers": &remove_thermometer.name
        }
    };

    rooms.update_one(query, update, None).await?;

    Ok(())
}
