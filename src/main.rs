use shop_scraper::routes::init_routes;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    init_routes(&mut app);

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
