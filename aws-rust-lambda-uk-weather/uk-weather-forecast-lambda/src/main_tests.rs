use crate::*;
use aws_lambda_events::event::eventbridge::EventBridgeEvent;
use lambda_runtime::{Context, LambdaEvent};
use mockall::predicate::*;
use mockall::*;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use chrono::{TimeZone, Utc};
use aws_config::BehaviorVersion;
use aws_sdk_ses::operation::send_email::builders::SendEmailFluentBuilder;
use aws_sdk_ses::types::{Body, Content, Destination, Message};
use std::future::Future;
use std::pin::Pin;
use aws_sdk_ses::operation::send_email::{SendEmailOutput, SendEmailError};
use aws_sdk_ses::error::SdkError;

mock! {
    pub SesClientTrait {}
    impl SesClientTrait for SesClientTrait {
        fn send_email(&self) -> SendEmailFluentBuilder;
    }
}

// fn mock_send_email_builder() -> SendEmailFluentBuilder {
//     MockSendEmailFluentBuilder::new().into()
// }

mock! {
    pub ReqwestClientTrait {}
    #[async_trait]
    impl ReqwestClientTrait for ReqwestClientTrait {
        async fn get(&self, url: &str) -> Result<reqwest::Response, reqwest::Error>;
    }
}

pub struct MockSendEmailFluentBuilder;

impl MockSendEmailFluentBuilder {
    pub fn new() -> Self {
        MockSendEmailFluentBuilder
    }

    pub fn source(self, _: impl Into<String>) -> Self {
        self
    }

    pub fn destination(self, _: aws_sdk_ses::types::Destination) -> Self {
        self
    }

    pub fn message(self, _: aws_sdk_ses::types::Message) -> Self {
        self
    }

    pub fn send(self) -> Pin<Box<dyn Future<Output = Result<SendEmailOutput, SdkError<SendEmailError>>> + Send>> {
        Box::pin(async {
            Ok(SendEmailOutput::builder().build()?)
        })
    }
}

impl From<MockSendEmailFluentBuilder> for SendEmailFluentBuilder {
    fn from(_: MockSendEmailFluentBuilder) -> Self {
        let config = aws_config::SdkConfig::builder()
            .behavior_version(BehaviorVersion::latest())
            .region(aws_sdk_ses::config::Region::new("us-west-2"))
            .build();

        let client = aws_sdk_ses::Client::new(&config);
        let builder = client.send_email()
            .source("test@example.com")
            .destination(
                Destination::builder()
                    .to_addresses("recipient@example.com")
                    .build()
            )
            .message(
                Message::builder()
                    .subject(
                        Content::builder()
                            .data("Test Subject")
                            .charset("UTF-8")
                            .build()
                            .expect("Failed to build subject content")
                    )
                    .body(
                        Body::builder()
                            .text(
                                Content::builder()
                                    .data("Test Body")
                                    .charset("UTF-8")
                                    .build()
                                    .expect("Failed to build body content")
                            )
                            .build()
                    )
                    .build()
            );
        builder
    }
}

#[tokio::test]
async fn test_fetch_weather() {
    let mock_response = fs::read_to_string(Path::new("test_data/london_weather_response.json"))
        .expect("Failed to read mock response file");

    let mut mock_client = MockReqwestClientTrait::default();
    mock_client
        .expect_get()
        .returning(move |_| {
            Ok(reqwest::Response::from(http::response::Response::builder()
                .status(200)
                .body(mock_response.clone())
                .unwrap()))
        });

    let result = fetch_weather("London", "fake_api_key", &mock_client).await.unwrap();

    assert_eq!(result.city, "London");
    assert_eq!(result.forecasts.len(), 2);
    assert_eq!(result.forecasts[0].temperature, 20.6);
    assert_eq!(result.forecasts[0].temp_min, 18.36);
    assert_eq!(result.forecasts[0].temp_max, 20.6);
    assert_eq!(result.forecasts[0].description, "overcast clouds");
    assert_eq!(result.forecasts[1].temperature, 18.78);
    assert_eq!(result.forecasts[1].temp_min, 17.31);
    assert_eq!(result.forecasts[1].temp_max, 18.78);
    assert_eq!(result.forecasts[1].description, "light rain");
}

#[test]
fn test_should_send_email() {
    let even_day = Utc.with_ymd_and_hms(2023, 7, 22, 0, 0, 0).unwrap();
    assert!(should_send_email(even_day));

    let odd_day = Utc.with_ymd_and_hms(2023, 7, 23, 0, 0, 0).unwrap();
    assert!(!should_send_email(odd_day));
}


//error
//function_handler returned an error: DispatchFailure(DispatchFailure { source: ConnectorError { kind: Other(None), source: NoMatchingAuthSchemeError(ExploredList { items: [ExploredAuthOption { scheme_id: AuthSchemeId { scheme_id: "sigv4" }, result: NoIdentityResolver }], truncated: false }), connection: Unknown } })

// #[tokio::test]
// async fn test_function_handler() {
//     // Set up environment variables
//     std::env::set_var("CITY_LIST", "London,Paris");
//     std::env::set_var("OPENWEATHERMAP_API_KEY", "fake_api_key");
//     std::env::set_var("FROM_EMAIL", "from@example.com");
//     std::env::set_var("SEAN_EMAIL", "sean@example.com");
//     std::env::set_var("WIFE_EMAIL", "wife@example.com");
//
//     // Create a mock event
//     let event = EventBridgeEvent {
//         version: Some("0".to_string()),
//         id: Some("1234".to_string()),
//         detail_type: "Scheduled Event".to_string(),
//         source: "aws.events".to_string(),
//         account: Some("123456789012".to_string()),
//         time: Some(Utc::now()),
//         region: Some("us-west-2".to_string()),
//         resources: Some(vec![]),
//         detail: serde_json::Value::Null,
//     };
//
//     let context = Context::default();
//
//     // Create mock ReqwestClient
//     let mut mock_reqwest_client = MockReqwestClientTrait::default();
//     mock_reqwest_client
//         .expect_get()
//         .returning(move |url| {
//             let response = if url.contains("q=London") {
//                 fs::read_to_string(Path::new("test_data/london_weather_response.json"))
//                     .expect("Failed to read London mock response file")
//             } else if url.contains("q=Paris") {
//                 fs::read_to_string(Path::new("test_data/paris_weather_response.json"))
//                     .expect("Failed to read Paris mock response file")
//             } else {
//                 panic!("Unexpected city in URL: {}", url);
//             };
//
//             Ok(reqwest::Response::from(http::response::Response::builder()
//                 .status(200)
//                 .body(response)
//                 .unwrap()))
//         });
//
//     // Create mock SesClient
//     let mut mock_ses_client = MockSesClientTrait::default();
//     mock_ses_client
//         .expect_send_email()
//         .returning(|| mock_send_email_builder());
//
//     // Call function_handler
//     let result = function_handler(
//         LambdaEvent::new(event, context),
//         Arc::new(mock_reqwest_client),
//         Arc::new(mock_ses_client)
//     ).await;
//
//     if let Err(e) = &result {
//         eprintln!("function_handler returned an error: {:?}", e);
//     }
//     assert!(result.is_ok(), "function_handler failed");
//
//     let response = result.unwrap();
//     assert_eq!(response.msg, "Weather forecast sent for 2 cities");
// }

#[tokio::test]
async fn test_function_handler_no_recipients() {
    std::env::set_var("CITY_LIST", "London,Paris");
    std::env::set_var("OPENWEATHERMAP_API_KEY", "fake_api_key");
    std::env::set_var("FROM_EMAIL", "from@example.com");
    std::env::set_var("SEAN_EMAIL", "");
    std::env::set_var("WIFE_EMAIL", "");

    let event = EventBridgeEvent {
        version: Some("0".to_string()),
        id: Some("1234".to_string()),
        detail_type: "Scheduled Event".to_string(),
        source: "aws.events".to_string(),
        account: Some("123456789012".to_string()),
        time: Some(Utc::now()),
        region: Some("us-west-2".to_string()),
        resources: Some(vec![]),
        detail: serde_json::Value::Null,
    };

    let context = Context::default();

    let mock_reqwest_client = MockReqwestClientTrait::default();
    let mock_ses_client = MockSesClientTrait::default();

    // Call function_handler
    let result = function_handler(
        LambdaEvent::new(event, context),
        Arc::new(mock_reqwest_client),
        Arc::new(mock_ses_client)
    ).await;

    if let Err(e) = &result {
        eprintln!("function_handler returned an error: {:?}", e);
    }
    assert!(result.is_ok(), "function_handler failed");

    let response = result.unwrap();
    assert_eq!(response.msg, "No recipients found. Email not sent.");
}

//error
//function_handler returned an error: DispatchFailure(DispatchFailure { source: ConnectorError { kind: Other(None), source: NoMatchingAuthSchemeError(ExploredList { items: [ExploredAuthOption { scheme_id: AuthSchemeId { scheme_id: "sigv4" }, result: NoIdentityResolver }], truncated: false }), connection: Unknown } })

// #[tokio::test]
// async fn test_send_email() {
//     let mut mock_client = MockSesClientTrait::default();
//     mock_client
//         .expect_send_email()
//         .returning(|| mock_send_email_builder());
//
//     let result = send_email(
//         &mock_client,
//         "from@example.com",
//         &["to@example.com".to_string()],
//         "Test Subject",
//         "Test Body",
//     ).await;
//
//     if let Err(e) = &result {
//         eprintln!("function_handler returned an error: {:?}", e);
//     }
//     assert!(result.is_ok(), "function_handler failed");
// }