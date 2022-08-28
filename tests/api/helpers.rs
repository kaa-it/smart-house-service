use bson::{doc, Document};
use mongodb::options::IndexOptions;
use mongodb::{Database, IndexModel};
use smart_house_service::configuration::{DatabaseSettings, Settings};
use smart_house_service::startup::{init_db, Application};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub port: u16,
}

impl TestApp {
    pub async fn add_room(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/api/v1/rooms", &self.address))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn add_power_switch(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/api/v1/power_switches", &self.address))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn remove_power_switch(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .delete(&format!("{}/api/v1/power_switches", &self.address))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app() -> TestApp {
    let configuration = {
        let mut c = Settings::new().expect("Failed to read configuration.");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };

    configure_database(&configuration.database)
        .await
        .expect("Failed to configure database");

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");

    let application_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address: format!("http://127.0.0.1:{}", application_port),
        port: application_port,
    }
}

async fn configure_database(config: &DatabaseSettings) -> anyhow::Result<()> {
    let db = init_db(config).await?;

    create_rooms_index(&db).await?;
    create_power_switches_index(&db).await?;

    Ok(())
}

async fn create_rooms_index(db: &Database) -> anyhow::Result<()> {
    let rooms = db.collection::<Document>("rooms");

    let options = IndexOptions::builder()
        .name(Some("name".to_string()))
        .unique(true)
        .build();

    let model = IndexModel::builder()
        .keys(doc! {"name": 1u32})
        .options(options)
        .build();

    rooms.create_index(model, None).await?;

    Ok(())
}

async fn create_power_switches_index(db: &Database) -> anyhow::Result<()> {
    let power_switches = db.collection::<Document>("power_switches");

    let options = IndexOptions::builder()
        .name(Some("nameAndRoomName".to_string()))
        .unique(true)
        .build();

    let model = IndexModel::builder()
        .keys(doc! {"name": 1u32, "room_name": 1u32})
        .options(options)
        .build();

    power_switches.create_index(model, None).await?;

    Ok(())
}
