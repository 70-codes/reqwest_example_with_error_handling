use reqwest::Error;
use tokio::task;

#[tokio::main]
async fn main() {
    let users = vec!["rust-lang", "tokio-rs", "async-rust"];
    let tasks: Vec<_> = users
        .iter()
        .map(|username| task::spawn(async { fetch_username_data(username).await }))
        .collect();

    for task in tasks {
        match task.await {
            Ok(Ok(user_data)) => {
                println!("\n {} \n", user_data);
            }
            Ok(Err(e)) => {
                println!("Error fetching user data: {}", e);
            }
            Err(e) => {
                println!("Error fetching user data: {}", e);
            }
        }
    }
    println!("");
    println!("Fetching user data completed.");
    println!("");
}

async fn fetch_username_data(username: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://api.github.com/users/{}", username);
    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .header("User-Agent", "tokio-tutorial")
        .send()
        .await?;
    let body = res.text().await?;
    Ok(body)
}
