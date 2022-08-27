use smart_house_service::configuration::Settings;
use smart_house_service::startup::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = Settings::new().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
