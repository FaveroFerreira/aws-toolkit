#!/usr/bin/env bash

echo "CREATING SQS QUEUE"

aws sqs create-queue --endpoint-url=http://localhost:4566 --queue-name testing_queue