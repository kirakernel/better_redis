use mini_redis::client;

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.01:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
