use std::{env, net::SocketAddr};

use axum::{Router, routing::{get, post}};

use crate::handlers::{credits::{get_credits_api, topup_credits_api}, donate::donate_api, payment::{create_payment_request_api, get_payment_status_api}, wallet::{get_balance_api, send_nano_api}};

pub async fn initialize_api_routers() {
    let app = set_routes();
    let listen_addr: SocketAddr = env::var("HTTP_LISTEN_ADDR")
        .expect("HTTP_LISTEN_ADDR not set")
        .parse()
        .expect("Invalid HTTP_LISTEN_ADDR format");

    println!("✅ API server listening on {}", listen_addr);

    let listener = tokio::net::TcpListener::bind(listen_addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

}

fn set_routes() -> Router {
    let app = Router::new()
        .route("/wallet/balance", get(get_balance_api))
        .route("/wallet/send", post(send_nano_api)) 
        .route("/payment/request", post(create_payment_request_api)) 
        .route("/payment/status/{transaction_id}", get(get_payment_status_api)) 
        .route("/credits", get(get_credits_api))
        .route("/credits/topup/{credits_amount}", post(topup_credits_api)) 
        .route("/donate/{amount}", post(donate_api)); 

    app
}

