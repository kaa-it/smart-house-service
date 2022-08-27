use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use crate::configuration::{DatabaseSettings, Settings};
use mongodb::{options::ClientOptions, options::ResolverConfig, Database, error::Error};
use paperclip::actix::{OpenApiExt,  web::{self}};
use crate::routes::rooms;

pub struct Application {
    port: u16,
    server: Server
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let db = init_db(&configuration.database).await?;

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, db)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn init_db(configuration: &DatabaseSettings) -> Result<Database, Error> {
    let client_uri = configuration.uri();

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;

    Ok(mongodb::Client::with_options(options)?.database("openers"))
}

fn run(listener: TcpListener, db: Database) -> Result<Server, std::io::Error> {
    let database = web::Data::new(db);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .wrap_api()
            .service(
                web::scope("/api/v1")
                    .route("/rooms", web::get().to(rooms))
            )
            .with_json_spec_at("/api/spec/v2")
            .with_swagger_ui_at("/docs")
            .build()
    })
        .listen(listener)?
        .run();

    Ok(server)
}