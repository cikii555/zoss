use axum::{routing::get, Router};
use scylla::{Session, SessionBuilder};
use std::sync::Arc;

async fn safe_transactions(session: Arc<Session>) -> String {

   

    let rows = session.query(
        "SELECT * FROM bank.transactions
         WHERE account_id=1
         AND tx_time > toTimestamp(now()) - 86400000
         LIMIT 100",
        &[]
    ).await.unwrap();

    format!("Safe query returned {}", rows.rows.unwrap_or_default().len())
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
            move || safe_transactions(s.clone())
        }));

    println!("Mitigated service running on 3001");

    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
