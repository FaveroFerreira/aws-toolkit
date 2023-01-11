#[tokio::main]
async fn main() {
    aws_toolkit::sqs::listen_messages().await;
}
