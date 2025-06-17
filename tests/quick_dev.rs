use anyhow::Result;

const ENDPOINT_URL: &str = "http://127.0.0.1:8080";

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client(ENDPOINT_URL)?;
    hc.do_get("/hello?name=shubhendu").await?.print().await?;
    Ok(())
}
