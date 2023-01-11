use rusoto_core::Region;
use rusoto_sqs::{ReceiveMessageRequest, Sqs, SqsClient};
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;

pub async fn listen_messages() {
    let sqs_client = SqsClient::new(Region::Custom {
        name: "us-east-2".to_string(),
        endpoint: "http://localhost:4566".to_string(),
    });

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("output/sqs_messages.json")
        .unwrap();

    loop {
        if let Ok(pool_response) = sqs_client
            .receive_message(ReceiveMessageRequest {
                queue_url: "http://localhost:4566/000000000000/testing_queue".to_string(),
                ..Default::default()
            })
            .await
        {
            if let Some(messages) = pool_response.messages {
                for message in messages {
                    if let Some(body) = message.body {
                        file.write(body.as_bytes()).unwrap();
                        file.write(",".as_bytes()).unwrap();
                    }
                }
            }
        }

        tokio::time::sleep(Duration::from_secs(3)).await;
    }
}
