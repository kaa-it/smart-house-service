use bson::Document;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::Database;
use crate::persistence::utils::check_already_exists;
use serde::{Serialize, Deserialize};
use futures::StreamExt;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewRoomEntity {
    pub name: String
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
    pub thermometers: Vec<String>
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