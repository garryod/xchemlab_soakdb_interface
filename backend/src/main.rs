#![doc = include_str!("../../README.md")]
#![forbid(unsafe_code)]
pub mod api;
pub mod models;

use api::{schema_builder, RootSchema};
use async_graphql::{extensions::Tracing, http::GraphiQLSource};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::{
    extract::{FromRef, State},
    headers::{authorization::Bearer, Authorization},
    response::{Html, IntoResponse},
    routing::get,
    Router, Server, TypedHeader,
};
use clap::Parser;
use opa_client::{AuthorizationToken, OPAClient};
use std::{
    fs::File,
    io::Write,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::PathBuf,
};
use url::Url;

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
    State(authz_client): State<OPAClient>,
    authorization_header: Option<TypedHeader<Authorization<Bearer>>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let token =
        AuthorizationToken::from(authorization_header.map(|header| header.token().to_string()));
    schema
        .execute(req.into_inner().data(token).data(authz_client))
        .await
        .into()
}

#[derive(Clone, FromRef)]
struct AppState {
    opa_client: OPAClient,
    schema: RootSchema,
}

fn setup_router(schema: RootSchema, opa_client: OPAClient) -> Router {
    Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .with_state(AppState { opa_client, schema })
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
            let schema = setup_api();
            let opa_client = OPAClient::new(args.opa_url);
            let router = setup_router(schema, opa_client);
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
