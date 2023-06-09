#![doc = include_str!("../../README.md")]
#![forbid(unsafe_code)]
pub mod api;
pub mod models;

use api::{schema_builder, RootSchema};
use async_graphql::{extensions::Tracing, http::GraphiQLSource};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use auth::{create_oidc_client, get_authentication_url, middleware::ExtractAuthToken, CoreClient};
use axum::{
    extract::{FromRef, State},
    response::{Html, IntoResponse},
    routing::get,
    Router, Server,
};
use clap::Parser;
use std::{
    fs::File,
    io::Write,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::PathBuf,
};

fn setup_api() -> RootSchema {
    schema_builder().extension(Tracing).finish()
}

async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

async fn graphql_handler(
    State(schema): State<RootSchema>,
    token: ExtractAuthToken,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();
    request.data.insert(token.into_inner().ok());
    schema.execute(request).await.into()
}

#[derive(Clone, FromRef)]
struct AppState {
    authn_client: CoreClient,
    schema: RootSchema,
}

fn setup_router(schema: RootSchema, authn_client: CoreClient) -> Router {
    Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .with_state(AppState {
            authn_client,
            schema,
        })
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
            let schema = setup_api();
            let authn_client = create_oidc_client()
                .await
                .expect("Failed to setup authentication client");
            println!("Authenticate at {}", get_authentication_url(&authn_client));
            let router = setup_router(schema, authn_client);
            serve(router, args.port).await;
        }
        Cli::Schema(args) => {
            let schema = setup_api();
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
