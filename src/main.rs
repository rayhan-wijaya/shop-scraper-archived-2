use shop_scraper::routes::init_routes;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenvy::dotenv()?;

    let mut app = tide::new();
    init_routes(&mut app);

    let listen_url = format!(
        "{}:{}",
        std::env::var("LISTEN_HOST")?,
        std::env::var("LISTEN_PORT")?
    );

    app.listen(listen_url).await?;

    return Ok(());
}
