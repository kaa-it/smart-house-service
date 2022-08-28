use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::power_switches::power_switches_config;
use crate::routes::reports::reports_config;
use crate::routes::rooms::rooms_config;
use crate::routes::thermometers::thermometers_config;
use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use mongodb::{error::Error, options::ClientOptions, options::ResolverConfig, Database};
use paperclip::actix::{
    web::{self},
    OpenApiExt,
};
use paperclip::v2::models::{Api, DefaultSchemaRaw, Info, Parameter, Response};
use std::net::TcpListener;

pub struct Application {
    port: u16,
    server: Server,
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
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;

    Ok(mongodb::Client::with_options(options)?.database(configuration.database_name.as_str()))
}

fn run(listener: TcpListener, db: Database) -> Result<Server, std::io::Error> {
    let database = web::Data::new(db);

    let spec = Api::<Parameter<DefaultSchemaRaw>, Response<DefaultSchemaRaw>, DefaultSchemaRaw> {
        info: Info {
            version: "0.1".into(),
            title: "Smart House Service".into(),
            ..Default::default()
        },
        ..Default::default()
    };

    let server = HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .wrap_api_with_spec(spec.clone())
            .service(
                web::scope("/api/v1")
                    .configure(rooms_config)
                    .configure(power_switches_config)
                    .configure(thermometers_config)
                    .configure(reports_config),
            )
            .with_json_spec_at("/api/spec/v2")
            .with_swagger_ui_at("/docs")
            .build()
    })
    .listen(listener)?
    .run();

    Ok(server)
}
