#[async_std::main]
async fn main() -> tide::Result<()> {
    let app = tide::new();
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
