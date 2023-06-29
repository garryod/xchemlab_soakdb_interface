#![doc=include_str!("../README.md")]
#![forbid(unsafe_code)]
mod graphql;
mod migrations;
mod resolvers;
mod tables;

use async_graphql::extensions::Tracing;
use axum::{routing::get, Router, Server};
use clap::Parser;
use graphql::{root_schema_builder, RootSchema};
use graphql_endpoints::{GraphQLHandler, GraphQLSubscription, GraphiQLHandler};
use migrations::Migrator;
use opa_client::OPAClient;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr, TransactionError};
use sea_orm_migration::MigratorTrait;
use std::{
    fs::File,
    io::Write,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::PathBuf,
};
use url::Url;

fn setup_router(schema: RootSchema) -> Router {
    const GRAPHQL_ENDPOINT: &str = "/";
    const SUBSCRIPTION_ENDPOINT: &str = "/ws";

    Router::new()
        .route(
            GRAPHQL_ENDPOINT,
            get(GraphiQLHandler::new(
                GRAPHQL_ENDPOINT,
                SUBSCRIPTION_ENDPOINT,
            ))
            .post(GraphQLHandler::new(schema.clone())),
        )
        .route_service(SUBSCRIPTION_ENDPOINT, GraphQLSubscription::new(schema))
}

async fn setup_database(database_url: Url) -> Result<DatabaseConnection, TransactionError<DbErr>> {
    let connection_options = ConnectOptions::new(database_url.to_string());
    let connection = Database::connect(connection_options).await?;
    Migrator::up(&connection, None).await?;
    Ok(connection)
}

async fn serve(router: Router, port: u16) {
    let socket_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port));
    println!("GraphiQL IDE: {}", socket_addr);
    Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[allow(clippy::large_enum_variant)]
enum Cli {
    /// Starts a webserver serving the GraphQL API
    Serve(ServeArgs),
    /// Prints the GraphQL API to stdout
    Schema(SchemaArgs),
}

#[derive(Debug, Parser)]
struct ServeArgs {
    /// The port number to serve on.
    #[arg(short, long, default_value_t = 80)]
    port: u16,
    /// The URL of a postgres database which will be used to persist service data.
    #[arg(long, env)]
    database_url: Url,
    /// The URL of an Open Policy Agent instance serving the required policy endpoints.
    #[arg(long, env)]
    opa_url: Url,
}

#[derive(Debug, Parser)]
struct SchemaArgs {
    /// The file path to write the schema to. If not supplied the schema will be printed to stdout.
    #[arg(short, long)]
    path: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let args = Cli::parse();

    let tracing_subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(tracing_subscriber).unwrap();

    match args {
        Cli::Serve(args) => {
            let opa_client = OPAClient::new(args.opa_url);
            let database = setup_database(args.database_url).await.unwrap();
            let schema = root_schema_builder()
                .extension(Tracing)
                .data(opa_client)
                .data(database)
                .finish();
            let router = setup_router(schema);
            serve(router, args.port).await;
        }
        Cli::Schema(args) => {
            let schema = root_schema_builder().finish();
            let schema_string = schema.sdl();
            if let Some(path) = args.path {
                let mut file = File::create(path).unwrap();
                file.write_all(schema_string.as_bytes()).unwrap();
            } else {
                println!("{}", schema_string);
            }
        }
    }
}
