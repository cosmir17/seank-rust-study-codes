#!/bin/bash

# Check if cargo-lambda is installed
if ! command -v cargo-lambda &> /dev/null
then
    echo "cargo-lambda is not installed. Please install it first."
    echo "You can install it using: cargo install cargo-lambda"
    exit 1
fi

# Run the lambda function with a sample EventBridgeEvent
echo "Invoking uk-weather-forecast-lambda locally..."
cargo lambda invoke \
  --data-ascii '{
    "version": "0",
    "id": "3333333-55555-3252-5555-222222222222",
    "detail-type": "Scheduled Event",
    "source": "aws.events",
    "account": "34534632523",
    "time": "2023-11-14T00:00:00Z",
    "region": "us-east-1",
    "resources": ["arn:aws:events:us-east-1:34534632523:rule/weather-forecast-rule"],
    "detail": {}
  }' \
  --output-format json \
  uk-weather-forecast-lambda

# Check the exit status
if [ $? -eq 0 ]; then
    echo "Lambda function invoked successfully."
else
    echo "There was an error invoking the lambda function."
    exit 1
fi