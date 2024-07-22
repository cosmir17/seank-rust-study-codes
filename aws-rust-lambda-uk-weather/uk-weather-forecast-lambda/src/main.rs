use serde::{Deserialize, Serialize};
use aws_sdk_ses::Client as SesClient;
use aws_sdk_ses::types::{Destination, Content, Body, Message};
use aws_lambda_events::event::eventbridge::EventBridgeEvent;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use aws_sdk_ses::operation::send_email::builders::SendEmailFluentBuilder;

use chrono::{DateTime, Utc, Datelike};
use reqwest::{Client as ReqwestClient};
use std::sync::Arc;

#[cfg(test)]
mod main_tests;

#[derive(Deserialize)]
struct Request {}

#[derive(Serialize)]
pub struct Response {
    pub req_id: String,
    pub msg: String,
}

#[derive(Deserialize)]
struct WeatherResponse {
    list: Vec<ForecastItem>,
}

#[derive(Deserialize)]
struct ForecastItem {
    dt: i64,
    main: MainWeather,
    weather: Vec<WeatherDescription>,
}

#[derive(Deserialize)]
struct MainWeather {
    temp: f32,
    temp_min: f32,
    temp_max: f32,
}

#[derive(Deserialize)]
struct WeatherDescription {
    description: String,
}

pub struct WeatherForecast {
    pub city: String,
    pub forecasts: Vec<DayForecast>,
}

pub struct DayForecast {
    pub date: String,
    pub temperature: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub description: String,
}

use async_trait::async_trait;

#[async_trait]
pub trait ReqwestClientTrait: Send + Sync {
    async fn get(&self, url: &str) -> Result<reqwest::Response, reqwest::Error>;
}

#[async_trait]
impl ReqwestClientTrait for ReqwestClient {
    async fn get(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        self.get(url).send().await
    }
}

pub trait SesClientTrait {
    fn send_email(&self) -> SendEmailFluentBuilder;
}

impl SesClientTrait for SesClient {
    fn send_email(&self) -> SendEmailFluentBuilder {
        self.send_email()
    }
}

pub async fn fetch_weather(city: &str, api_key: &str, client: &dyn ReqwestClientTrait) -> Result<WeatherForecast, Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?q={}&appid={}&units=metric&cnt=16",
        city, api_key
    );
    let response = client.get(&url)
        .await
        .map_err(|e| Error::from(format!("Failed to fetch weather data. request url : {}, error: {}", &url, e)))?;

    let response_text = response.text()
        .await
        .map_err(|e| Error::from(format!("Failed to get response text. request url : {}, error: {}", &url, e)))?;

    let weather_response: WeatherResponse = serde_json::from_str(&response_text)
        .map_err(|e| Error::from(format!("Failed to parse weather data. request url : {}, error: {}, body: {}", &url, e, response_text)))?;

    let mut forecasts = Vec::new();
    for item in weather_response.list.iter().take(2) {
        let date = DateTime::<Utc>::from_timestamp(item.dt, 0)
            .unwrap_or_default()
            .format("%Y-%m-%d")
            .to_string();
        forecasts.push(DayForecast {
            date,
            temperature: item.main.temp,
            temp_min: item.main.temp_min,
            temp_max: item.main.temp_max,
            description: item.weather[0].description.clone(),
        });
    }

    Ok(WeatherForecast {
        city: city.to_string(),
        forecasts,
    })
}

pub async fn send_email(client: &dyn SesClientTrait, from: &str, to: &[String], subject: &str, body: &str) -> Result<(), Error> {
    let destination = Destination::builder()
        .set_to_addresses(Some(to.to_vec()))
        .build();

    let subject_content = Content::builder()
        .data(subject)
        .charset("UTF-8")
        .build();

    let body_content = Content::builder()
        .data(body)
        .charset("UTF-8")
        .build();

    let body = Body::builder()
        .text(body_content.unwrap())
        .build();

    let message = Message::builder()
        .subject(subject_content.unwrap())
        .body(body)
        .build();

    client.send_email()
        .source(from)
        .destination(destination)
        .message(message)
        .send()
        .await?;

    Ok(())
}

pub fn should_send_email(now: DateTime<Utc>) -> bool {
    now.day() % 2 == 0
}

pub async fn function_handler(
    event: LambdaEvent<EventBridgeEvent>,
    reqwest_client: Arc<dyn ReqwestClientTrait>,
    ses_client: Arc<dyn SesClientTrait>,
) -> Result<Response, Error> {
    if !should_send_email(Utc::now()) {
        return Ok(Response {
            req_id: event.context.request_id,
            msg: "Skipping email send today".to_string(),
        });
    }

    let cities = std::env::var("CITY_LIST")
        .map_err(|_| Error::from("CITY_LIST environment variable not set"))?
        .replace("\"", "")
        .to_lowercase()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();

    if cities.is_empty() {
        return Ok(Response {
            req_id: event.context.request_id,
            msg: "No cities specified in CITY_LIST".to_string(),
        });
    }

    let mut forecasts = Vec::new();

    let api_key = std::env::var("OPENWEATHERMAP_API_KEY")
        .map_err(|_| Error::from("OPENWEATHERMAP_API_KEY environment variable not set"))?;

    let from = std::env::var("FROM_EMAIL")
        .map_err(|_| Error::from("FROM_EMAIL environment variable not set"))?;

    let mut to = Vec::new();
    if let Ok(sean_email) = std::env::var("SEAN_EMAIL") {
        if !sean_email.trim().is_empty() {
            to.push(sean_email);
        }
    }
    if let Ok(wife_email) = std::env::var("WIFE_EMAIL") {
        if !wife_email.trim().is_empty() {
            to.push(wife_email);
        }
    }

    if to.is_empty() {
        return Ok(Response {
            req_id: event.context.request_id,
            msg: "No recipients found. Email not sent.".to_string(),
        });
    }

    for city in &cities {
        let forecast = fetch_weather(city, &api_key, reqwest_client.as_ref()).await?;
        forecasts.push(forecast);
    }

    let subject = format!("Weather Forecast for {} - {}", cities.join(", "), Utc::now().format("%Y-%m-%d"));

    let body = forecasts.iter()
        .map(|f| format!("{}\n{}: {:.1}°C (Min: {:.1}°C, Max: {:.1}°C), {}\n{}: {:.1}°C (Min: {:.1}°C, Max: {:.1}°C), {}",
                         f.city,
                         f.forecasts[0].date, f.forecasts[0].temperature, f.forecasts[0].temp_min, f.forecasts[0].temp_max, f.forecasts[0].description,
                         f.forecasts[1].date, f.forecasts[1].temperature, f.forecasts[1].temp_min, f.forecasts[1].temp_max, f.forecasts[1].description))
        .collect::<Vec<String>>()
        .join("\n\n");

    send_email(ses_client.as_ref(), &from, &to, &subject, &body).await?;

    Ok(Response {
        req_id: event.context.request_id,
        msg: format!("Weather forecast sent for {} cities", cities.len()),
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let reqwest_client: Arc<dyn ReqwestClientTrait> = Arc::new(ReqwestClient::new());
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let ses_client: Arc<dyn SesClientTrait> = Arc::new(SesClient::new(&config));

    run(service_fn(|event| function_handler(event, reqwest_client.clone(), ses_client.clone()))).await
}