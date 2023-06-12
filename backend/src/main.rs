#![doc = include_str!("../../README.md")]
#![forbid(unsafe_code)]
pub mod api;
pub mod models;

use api::{schema_builder, RootSchema};
use async_graphql::{extensions::Tracing, http::GraphiQLSource};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use auth::{authentication::OIDCClient, authorization::OPAClient, middleware::ExtractAuthToken};
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
    token: ExtractAuthToken,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema
        .execute(
            req.into_inner()
                .data(token.into_inner().ok())
                .data(authz_client),
        )
        .await
        .into()
}

#[derive(Clone, FromRef)]
struct AppState {
    authn_client: OIDCClient,
    authz_client: OPAClient,
    schema: RootSchema,
}

fn setup_router(schema: RootSchema, authn_client: OIDCClient, authz_client: OPAClient) -> Router {
    Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .with_state(AppState {
            authn_client,
            authz_client,
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
    /// The URL of the OpenID Connect authentiation service.
    #[arg(long, env)]
    oidc_issuer_url: Url,
    /// The OpenID Connect Client ID of this application.
    #[arg(long, env)]
    oidc_client_id: String,
    /// The OpenID Connect Client Secret of this application.
    #[arg(long, env)]
    oidc_client_secret: Option<String>,
    /// The URL to redirect the user to after OpenID Connect authentication.
    #[arg(long, env)]
    oidc_redirect_url: Url,
    /// The URL of an Access Token introspection endpoint.
    #[arg(long, env)]
    access_token_introspection_url: Url,
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
            let authn_client = OIDCClient::new(
                args.oidc_issuer_url,
                &args.oidc_client_id,
                args.oidc_client_secret.as_ref(),
                args.access_token_introspection_url,
            )
            .await
            .unwrap();
            let authz_client = OPAClient::new(args.opa_url);
            println!(
                "Authenticate at {}",
                authn_client.authentication_url(args.oidc_redirect_url)
            );
            let router = setup_router(schema, authn_client, authz_client);
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
