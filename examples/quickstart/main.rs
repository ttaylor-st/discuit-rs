use discuit_rs::client::*;

#[tokio::main]
async fn main() {
    let base_url = "https://discuit.net";
    let mut client = DiscuitClient::new(base_url);
    let response = client.initialize().await;
    println!("{:?}", response);
}
