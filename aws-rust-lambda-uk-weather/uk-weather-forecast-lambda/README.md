# Weather Forecast Lambda

This is a Rust-based AWS Lambda function that fetches weather forecasts for specified cities and sends the information via email every two days.

## Features

- Fetches two-day weather forecasts for a list of cities using the OpenWeatherMap API
- Includes current temperature, minimum temperature, and maximum temperature for each day
- Sends formatted email reports with detailed weather information
- Designed to run every two days (scheduling configured separately)
- Uses AWS Simple Email Service (SES) for sending emails

## Prerequisites

- Rust and Cargo installed (latest stable version)
- `cargo-lambda` installed (`cargo install cargo-lambda`)
- An AWS account with access to Lambda and SES
- OpenWeatherMap API key

## Setup

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/weather-forecast-lambda.git
   cd weather-forecast-lambda
   ```

2. Build the project:
   ```
   cargo lambda build --release
   ```

3. Set up the following environment variables (these will be configured in the Lambda function):
   - `OPENWEATHERMAP_API_KEY`: Your OpenWeatherMap API key
   - `FROM_EMAIL`: The email address to send from (must be verified in SES)
   - `SEAN_EMAIL`: Your email address (optional)
   - `WIFE_EMAIL`: Your wife's email address (optional)
   - `CITY_LIST`: Comma-separated list of cities to fetch weather for (e.g., "London,New York,Tokyo")

   Note: The `SEAN_EMAIL` and `WIFE_EMAIL` variables are optional. If either is not set or is an empty string, the email will not be sent to that recipient. This allows for flexible configuration of email recipients without modifying the code.

## Email Content

The email sent by this Lambda function will include the following information for each city:

- City name
- For each of the next two days:
   - Date
   - Current temperature
   - Minimum temperature
   - Maximum temperature
   - Weather description

This provides a comprehensive overview of the expected weather conditions for the specified cities.

## Local Testing

To test the function locally:

1. Set up the environment variables mentioned above. You can use the `run_local.sh` script to set these variables and run the function in debug mode:
   ```
   ./run_local.sh
   ```

   Make sure to edit `run_local.sh` to include the correct email addresses, API key, and city list.

2. Use `cargo lambda` to invoke the function locally:
   ```
   ./invoke_local_lambda.sh
   ```

## Deployment

1. Build the release version of your Lambda function:
   ```
   cargo lambda build --release
   ```

2. Deploy the function using `cargo lambda`:
   ```
   cargo lambda deploy --verbose --iam-role arn:aws:iam::11111111111:role/cargo-lambda-role-bbbbbbbbb-dddd-22222-dddd-fffffffffffff
   ```

   Note: Make sure to replace the IAM role ARN with the appropriate role for your AWS account.

3. For ARM64 architecture, use:
   ```
   cargo lambda build --release --arm64
   ```

4. After deployment, make sure to configure the environment variables in the AWS Lambda console or using the AWS CLI.

## Invoking the Deployed Function

To invoke the deployed function, use:

```
cargo lambda invoke --remote \
  --data-ascii '{
    "version": "0",
    "id": "sdfsdfsdf-ewefwfe-sdfsdf-sgdgsdg-wegwgweg",
    "detail-type": "Scheduled Event",
    "source": "aws.events",
    "account": "2352362362",
    "time": "2023-11-14T00:00:00Z",
    "region": "us-east-1",
    "resources": ["arn:aws:events:us-east-1:234235235232:rule/weather-forecast-rule"],
    "detail": {}
  }' \
  --output-format json \
  weather-forecast-lambda
```

## Logging

To view logs when invoking the Lambda function, use the `--log-type Tail` option with the AWS CLI:

```
aws lambda invoke --function-name weather-forecast-lambda \
  --payload '{"key1": "value1", "key2": "value2"}' \
  --log-type Tail \
  --query 'LogResult' \
  --output text \
  output.txt | base64 --decode
```

## Development

To add new features or modify existing ones, edit the `src/main.rs` file. Make sure to update the tests in `src/main_tests.rs` accordingly.

To run tests:
```
cargo test
```
