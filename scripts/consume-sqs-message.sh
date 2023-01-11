#!/usr/bin/env bash

echo "CONSUME SQS QUEUE"

aws sqs receive-message \
--endpoint-url=http://localhost:4566 \
--queue-url http://localhost:4566/000000000000/testing_queue \
--attribute-names All \
--message-attribute-names All \
--max-number-of-messages 10
