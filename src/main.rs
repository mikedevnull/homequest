use homequest::app;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    app::run().await
}
