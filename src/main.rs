#[tokio::main]
async fn main() {
    tokio::task::consume_budget().await;
    println!("Hello, world!");
}
