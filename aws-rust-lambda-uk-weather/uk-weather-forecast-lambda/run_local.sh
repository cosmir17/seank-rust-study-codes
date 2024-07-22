#!/bin/bash

# Set environment variables
export OPENWEATHERMAP_API_KEY=mykeymykey
export FROM_EMAIL=my_email@gmail.com
export SEAN_EMAIL=my_email@gmail.com
export WIFE_EMAIL=your_wife_email@example.com
export CITY_LIST="Paris,London"

# Check if cargo-lambda is installed
if ! command -v cargo-lambda &> /dev/null
then
    echo "cargo-lambda is not installed. Please install it first."
    echo "You can install it using: cargo install cargo-lambda"
    exit 1
fi

# Run the lambda function in debug mode
echo "Running weather-forecast-lambda in debug mode..."
cargo lambda watch

# Check the exit status
if [ $? -eq 0 ]; then
    echo "Lambda function executed successfully in debug mode."
else
    echo "There was an error running the lambda function in debug mode."
    exit 1
fi