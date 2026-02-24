use axum::{routing::get, Router};
use scylla::{Session, SessionBuilder};
use std::sync::Arc;

async fn get_transactions(session: Arc<Session>) -> String {

    let rows = session.query(
        "SELECT * FROM bank.transactions WHERE account_id=1",
        &[]
    ).await.unwrap();

    format!("Returned {} rows", rows.rows.unwrap_or_default().len())
}

#[tokio::main]
async fn main() {

    let session = Arc::new(
        SessionBuilder::new()
            .known_node("127.0.0.1:9042")
            .build()
            .await
            .unwrap()
    );

    let app = Router::new()
        .route("/transactions", get({
            let s = session.clone();
            move || get_transactions(s.clone())
        }));

    println!("Vulnerable service running on 3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}