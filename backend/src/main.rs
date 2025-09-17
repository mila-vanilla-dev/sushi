//! CLI interface for the UPS API client
#![allow(dead_code)]

use axum::Router;
use clap::Parser;
use dotenvy::dotenv;
use std::sync::Arc;
use sushi::{AppState, Result as UpsResult, UpsClient, UpsConfig, endpoints, middleware};
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sqlx::{postgres::PgPoolOptions};

/// SUSHI - UPS Address Validation Tool
#[derive(Parser, Debug)]
#[command(name = "sushi")]
#[command(about = "A tool for validating addresses and getting shipping rates using the UPS API")]
struct Args {
    /// Enable debug logging for raw API responses
    #[arg(long)]
    debug: bool,

    /// Path to ship-from configuration file
    #[arg(long, default_value = "sample-ship-dev.json")]
    ship_from: String,

    /// Path to order data file
    #[arg(long, default_value = "sample-order-dev.json")]
    order: String,
}

#[tokio::main]
async fn main() -> UpsResult<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Load environment variables from .env file
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to connect to Postgres");

    // Initialize tracing subscriber for structured logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                if args.debug {
                    format!(
                        "{}=debug,tower_http=debug,axum::rejection=trace",
                        env!("CARGO_CRATE_NAME")
                    )
                } else {
                    format!("{}=info,tower_http=info", env!("CARGO_CRATE_NAME"))
                }
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    if args.debug {
        tracing::info!("🐛 DEBUG mode enabled - raw API responses will be logged");
    }

    // Load configuration
    let config = UpsConfig::from_env().map_err(sushi::error::UpsError::Config)?;

    if args.debug {
        config.display();
    }

    // Throw a fit if JWT_SECRET is not set
    let _ = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    tracing::info!("Starting TPS Orders API server");

    // Create UPS client and get access token
    let client = UpsClient::new(config).with_debug(args.debug);
    tracing::info!("Authenticating with UPS API...");
    let access_token = client.get_access_token().await?;
    tracing::info!("✅ Successfully authenticated with UPS API");

    // Create application state with bootstrap admin
    let user_store = Arc::new(RwLock::new(endpoints::auth::UserStore::new_with_admin()));
    let app_state = AppState {
    ups_client: client,
    access_token,
    user_store,
    db_pool, 
    };

    // Startup axum server with tracing middleware
    let app = Router::new()
        .route(
            "/",
            axum::routing::get(|| async {
                tracing::info!("Health check endpoint called");
                "Hello, World!"
            }),
        )
        // Public authentication routes (no auth required)
        .route(
            "/api/auth/register",
            axum::routing::post(endpoints::auth::register_endpoint),
        )
        .route(
            "/api/auth/login",
            axum::routing::post(endpoints::auth::login_endpoint),
        )
        .route(
            "/api/auth/logout",
            axum::routing::post(endpoints::auth::logout_endpoint),
        )
        .route(
            "/api/auth/forgot-password",
            axum::routing::post(endpoints::auth::forgot_password_endpoint),
        )
        .route(
            "/api/auth/reset-password",
            axum::routing::post(endpoints::auth::reset_password_endpoint),
        )
        // Protected routes (require authentication)
        .nest(
            "/api",
            Router::new()
                .route("/auth/me", axum::routing::get(endpoints::auth::me_endpoint))
                .route(
                    "/users/{id}",
                    axum::routing::get(endpoints::auth::get_user_endpoint)
                        .patch(endpoints::auth::update_user_endpoint)
                        .delete(endpoints::auth::delete_user_endpoint),
                )
                .route(
                    "/users/{id}/password",
                    axum::routing::patch(endpoints::auth::update_password_endpoint),
                )
                .route(
                    "/orders",
                    axum::routing::post(endpoints::orders::orders_endpoint),
                )
                .layer(axum::middleware::from_fn(middleware::auth_middleware)),
        )
        // Admin-only routes
        .nest(
            "/api",
            Router::new()
                .route(
                    "/users",
                    axum::routing::get(endpoints::auth::list_users_endpoint),
                )
                .route(
                    "/users/{id}/role",
                    axum::routing::patch(endpoints::auth::update_user_role_endpoint),
                )
                .route(
                    "/admin/create-admin",
                    axum::routing::post(endpoints::admin::create_admin_endpoint),
                )
                .layer(axum::middleware::from_fn(middleware::admin_middleware)),
        )
        .route("/db_health", axum::routing::get(endpoints::db::db_health))
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    tower_http::trace::DefaultMakeSpan::new().level(tracing::Level::INFO),
                )
                .on_response(
                    tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO),
                ),
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
