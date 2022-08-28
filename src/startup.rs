use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use crate::configuration::{DatabaseSettings, Settings};
use mongodb::{options::ClientOptions, options::ResolverConfig, Database, error::Error};
use paperclip::actix::{OpenApiExt,  web::{self}};
use crate::routes::rooms::{rooms_config};
use paperclip::v2::models::{DefaultApiRaw, Info};

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

    Ok(mongodb::Client::with_options(options)?.database(configuration.database_name.as_str()))
}

fn run(listener: TcpListener, db: Database) -> Result<Server, std::io::Error> {
    let database = web::Data::new(db);

    let mut spec = DefaultApiRaw::default();

    spec.info = Info {
        version: "0.1".into(),
        title: "Smart House Service".into(),
        ..Default::default()
    };

    let server = HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .wrap_api_with_spec(spec.clone())
            .service(
                web::scope("/api/v1")
                    .configure(rooms_config)
            )
            .with_json_spec_at("/api/spec/v2")
            .with_swagger_ui_at("/docs")
            .build()
    })
        .listen(listener)?
        .run();

    Ok(server)
}