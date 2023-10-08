use tokio::time::Duration;

async fn fetch_data() -> Result<String, Box<dyn std::error::Error>> {
    // Simulate an asynchronous operation, e.g., fetching data from a remote server.
    tokio::time::sleep(Duration::from_secs(2)).await;
    Ok("Data from the server".to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = fetch_data().await?;

    println!("Fetched data: {}", result);

    Ok(())
}
