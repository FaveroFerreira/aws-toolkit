#!/usr/bin/env bash

echo "SENDING SQS MESSAGE"

aws sqs send-message \
--endpoint-url=http://localhost:4566 \
--queue-url http://localhost:4566/000000000000/testing_queue \
--message-body '{ "is_test": true }'