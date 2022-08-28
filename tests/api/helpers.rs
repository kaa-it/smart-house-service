use uuid::Uuid;
use smart_house_service::configuration::Settings;
use smart_house_service::startup::Application;

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
}

pub async fn spawn_app() -> TestApp {
    let configuration = {
        let mut c = Settings::new().expect("Failed to read configuration.");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };

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
