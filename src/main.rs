use axum_x_twitter_clone::App;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let port = std::env::var("PORT")
        .expect("Missing env PORT")
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let app = App::new(port);

    app.run().await.expect("Error running server");
}
