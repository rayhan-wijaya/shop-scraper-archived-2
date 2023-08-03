use shop_scraper::routes::init_routes;
use std::env::var;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenvy::dotenv()?;

    let mut app = tide::new();
    init_routes(&mut app);

    let listen_url = format!(
        "{}:{}",
        var("LISTEN_HOST")?,
        var("LISTEN_PORT")?
    );

    app.listen(listen_url).await?;

    return Ok(());
}
