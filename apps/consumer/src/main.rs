#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    println!("Scandium Kafka consumer starting...");
}
