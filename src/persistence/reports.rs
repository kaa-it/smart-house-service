use crate::persistence::power_switches::NewPowerSwitchEntity;
use crate::persistence::rooms::RoomEntity;
use crate::persistence::thermometers::NewThermometerEntity;
use bson::doc;
use futures::StreamExt;
use mongodb::Database;

pub async fn report_all(db: &Database) -> anyhow::Result<String> {
    let mut report = String::new();

    let rooms = db.collection::<RoomEntity>("rooms");

    let filter = bson::Document::new();

    let mut cursor = rooms.find(filter, None).await?;

    while let Some(room) = cursor.next().await {
        let room = room?;
        report_room(db, &room, &mut report).await?;
    }

    Ok(report)
}

async fn report_room(db: &Database, room: &RoomEntity, report: &mut String) -> anyhow::Result<()> {
    let line = format!("Room (name: '{}'):\n", room.name);
    report.push_str(&line);

    report_power_switches(db, room, report).await?;
    report_thermometers(db, room, report).await?;

    Ok(())
}

async fn report_power_switches(
    db: &Database,
    room: &RoomEntity,
    report: &mut String,
) -> anyhow::Result<()> {
    let power_switches = db.collection::<NewPowerSwitchEntity>("power_switches");

    let filter = doc! {
        "room_name": &room.name
    };

    let mut cursor = power_switches.find(filter, None).await?;
    while let Some(power_switch) = cursor.next().await {
        let power_switch = power_switch?;
        report_power_switch(&power_switch, report);
    }

    Ok(())
}

fn report_power_switch(power_switch: &NewPowerSwitchEntity, report: &mut String) {
    let line = format!(
        "    Power Switch (name: '{}', description: '{}', power_consumption: '{}')\n",
        power_switch.name, power_switch.description, power_switch.power_consumption
    );
    report.push_str(&line);
}

async fn report_thermometers(
    db: &Database,
    room: &RoomEntity,
    report: &mut String,
) -> anyhow::Result<()> {
    let thermometers = db.collection::<NewThermometerEntity>("thermometers");

    let filter = doc! {
        "room_name": &room.name
    };

    let mut cursor = thermometers.find(filter, None).await?;
    while let Some(thermometer) = cursor.next().await {
        let thermometer = thermometer?;
        report_thermometer(&thermometer, report);
    }

    Ok(())
}

fn report_thermometer(thermometer: &NewThermometerEntity, report: &mut String) {
    let line = format!(
        "    Thermometer (name: '{}', description: '{}', temperature: '{}')\n",
        thermometer.name, thermometer.description, thermometer.temperature
    );
    report.push_str(&line);
}
