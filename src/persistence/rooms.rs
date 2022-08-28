use crate::persistence::utils::check_already_exists;
use bson::doc;
use bson::oid::ObjectId;
use bson::Document;
use futures::StreamExt;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use crate::persistence::error::Error::NotFoundError;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewRoomEntity {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveRoomEntity {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RoomEntity {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    /// Room name
    pub name: String,

    /// List of power switch names
    pub power_switches: Vec<String>,

    /// List of thermometer names
    pub thermometers: Vec<String>,
}

pub async fn add_room(db: &Database, new_room: &NewRoomEntity) -> anyhow::Result<RoomEntity> {
    let docs = db.collection::<Document>("rooms");

    let room = doc! {
        "name": &new_room.name,
        "power_switches": [],
        "thermometers": []
    };

    let result = match docs.insert_one(room, None).await {
        Err(e) => return Err(check_already_exists(e)),
        Ok(result) => result,
    };

    let rooms = db.collection::<RoomEntity>("rooms");

    let r = rooms
        .find_one(doc! { "_id": result.inserted_id }, None)
        .await;

    let r = r?;

    Ok(r.unwrap())
}

pub async fn rooms(db: &Database) -> anyhow::Result<Vec<RoomEntity>> {
    let rooms = db.collection::<RoomEntity>("rooms");

    let filter = bson::Document::new();

    let mut cursor = rooms.find(filter, None).await?;

    let mut rooms: Vec<RoomEntity> = Vec::new();
    while let Some(room) = cursor.next().await {
        rooms.push(room?);
    }

    Ok(rooms)
}

pub async fn remove_room(db: &Database, remove_room: &RemoveRoomEntity) -> anyhow::Result<()> {
    let rooms = db.collection::<Document>("rooms");

    let room_filter = doc! {
        "name": &remove_room.name
    };

    let room = rooms.find_one(room_filter.clone(), None).await?;

    if room.is_none() {
        return Err(NotFoundError.into());
    }

    let power_switches = db.collection::<Document>("power_switches");

    let power_switch_filter = doc! {
        "room_name": &remove_room.name
    };

    power_switches.delete_many(power_switch_filter, None).await?;

    let thermometers = db.collection::<Document>("thermometers");

    let thermometer_filter = doc! {
        "room_name": &remove_room.name
    };

    thermometers.delete_many(thermometer_filter, None).await?;

    rooms.delete_one(room_filter, None).await?;

    Ok(())
}
