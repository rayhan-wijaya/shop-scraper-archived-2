use shop_scraper::routes::init_routes;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenvy::dotenv()?;

    let mut app = tide::new();
    init_routes(&mut app);

    let listen_host = std::env::var("LISTEN_HOST")?;
    let listen_address = std::env::var("LISTEN_ADDRESS")?;
    let listen_url = format!("{}:{}", listen_host, listen_address);

    app.listen(listen_url).await?;

    Ok(())
}
