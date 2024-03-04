mod model;
mod web;
use tokio;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(&url)
    //     .await
    //     .unwrap_or_else(|_| panic!("Failed to create Postgres connection pool! URL: {}", url));

    // match sqlx::migrate!("./migrations").run(&pool).await {
    //     Ok(_) => println!("Migrations ran successfully"),
    //     Err(e) => println!("Migrations failed: {}", e),
    // }

    let Ok(port) = std::env::var("IOT_ORCHID_PORT") else {
        panic!("IOT_ORCHID_PORT must be set.");
    };

    let Ok(address) = std::env::var("IOT_ORCHID_ADDRESS") else {
        panic!("IOT_ORCHID_ADDRESS must be set.");
    };

    let addr = format!("{}:{}", address, port);

    let app = match web::get_routes().await {
        Ok(app) => app,
        Err(e) => panic!("Failed to create routes: {}", e),
    };
    

    let listener = tokio::net::TcpListener::bind(&addr).await
        .expect("Failed to bind to address");

    axum::serve(listener, app).await
        .expect("Failed to start server");

}