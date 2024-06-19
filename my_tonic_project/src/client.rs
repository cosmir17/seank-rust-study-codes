#![recursion_limit = "512"]

use std::sync::Arc;
use async_channel::{Receiver, Sender, unbounded};
use tonic::metadata::MetadataValue;
use tonic::{Request, Status};
use tonic::transport::Channel;
use tokio::sync::{OnceCell};
use tonic::codegen::InterceptedService;
use tonic::service::Interceptor;
use tokio::time::{Duration, interval};
use log::{debug, info, warn};

pub mod proto {
    tonic::include_proto!("service");
}

mod config;

use proto::controller_client::ControllerClient;
use proto::ClientMsg;
use crate::proto::server_msg::{Data, SimOver, SimStart, SimUpdate};
use crate::proto::Area;

#[derive(Clone)]
struct ServiceState {
    x: f32,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct ServiceInstruction {
    b_minimum_x: f32,
    b_maximum_x: f32,
    g_minimum_x: f32,
    g_maximum_x: f32,
}

struct MyInterceptor {
    token: String
}

impl Interceptor for MyInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let token: MetadataValue<_> = self.token.parse()
            .map_err(|e| Status::invalid_argument(format!("Invalid metadata value: {:?}", e)))?;
        request.metadata_mut().insert("authorization", token.clone());
        Ok(request)
    }
}

fn calculate_control_values(service_state: &ServiceState, prev_service_state: &ServiceState, instruction: &ServiceInstruction, dt: f32) -> i32 {
    // there was some algorithm here. I deleted it to anonymise the code and hard coded the return value
    let g_x = (instruction.g_minimum_x + instruction.g_maximum_x) / 2.0;
    let distance_x = g_x - service_state.x;
    distance_x as i32
}


async fn establish_connection(
    state_receiver: Arc<Receiver<Option<ServiceState>>>,
    server_instruction: Arc<OnceCell<ServiceInstruction>>,
    navigation_success: Arc<OnceCell<bool>>,
    config: config::Config,
) -> Result<(ControllerClient<InterceptedService<Channel, MyInterceptor>>, tonic::Streaming<proto::ServerMsg>), Box<dyn std::error::Error>> {
    let channel = Channel::from_shared(config.endpoint.to_string())?.connect().await?;
    let mut client = ControllerClient::with_interceptor(channel, MyInterceptor { token: config.token.to_string() } );
    let delta_time = 0.02;

    let outbound = async_stream::stream! {
        let mut interval = interval(Duration::from_secs_f32(delta_time));
        let mut prev_service_state = ServiceState { x: 0.0 };

        while let (None, server_instruction, Ok(service_state_maybe)) =
        (navigation_success.get(), server_instruction.get(), state_receiver.recv().await) {
            interval.tick().await;

            match (service_state_maybe, server_instruction) {
                (Some(service_state), Some(instruction)) => {
                    let c = calculate_control_values(&service_state, &prev_service_state, &instruction, delta_time);
                    prev_service_state = service_state;
                    debug!("both information present: c:{:?}", c);
                    yield ClientMsg { c };
                },
                _ => {
                    debug!("Not supposed to happen but returning empty Service Client Message ");
                    yield ClientMsg { c: 0 };
                },
            }
        }
    };

    let response = client.connection(Request::new(outbound)).await?;
    let inbound = response.into_inner();

    Ok((client, inbound))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Endpoint: {}", config::CONFIG.endpoint);
    info!("Token: {}", config::CONFIG.token);

    let empty_string = String::from("");
    loop {
        let (state_sender, state_receiver): (Sender<Option<ServiceState>>, Receiver<Option<ServiceState>>) = unbounded();
        let state_receiver_arc = Arc::new(state_receiver);
        let server_instruction = Arc::new(OnceCell::new());
        let navigation_success = Arc::new(OnceCell::new());
        let (_, mut inbound) = establish_connection(state_receiver_arc, server_instruction.clone(), navigation_success.clone(), config::CONFIG).await?;

        while let Some(server_msg) = inbound.message().await? {
            match &server_msg.data {
                Some(Data::Start(SimStart {
                                     l: Some(l),
                                     limit: Some(Area { n_point: Some(boundary_min), m_point: Some(boundary_max) }),
                                     g: Some(Area { n_point: Some(goal_min), m_point: Some(goal_max) })})) => {
                    let state = ServiceState { x: l.x };

                    let service_instruction = ServiceInstruction {
                        b_minimum_x: boundary_min.x,
                        b_maximum_x: boundary_max.x,
                        g_minimum_x: goal_min.x,
                        g_maximum_x: goal_max.x,
                    };

                    server_instruction.set(service_instruction).unwrap();
                    state_sender.send(Some(state)).await.unwrap_or_else(|e| warn!("async-channel error, SendError is {:?}", e.to_string()));
                }

                Some(Data::Start(SimStart { l: _, limit: _, g: _, })) => {
                    panic!("this shouldn't happen, SimStart should provide all the necessary instructions needed");
                }
                Some(Data::Update(SimUpdate { l: Some(location) })) => {
                    let state = ServiceState { x: location.x, };
                    state_sender.send(Some(state)).await?;
                }
                Some(Data::Ended(SimOver { success: true, details })) => {
                    info!("\n\n******** Simulation ended with success ******** {:?}\n", details.as_ref().unwrap_or(&empty_string));
                    navigation_success.set(true).unwrap();
                    break;
                }
                Some(Data::Ended(SimOver { success: false, details })) => {
                    info!("\n\n******** Simulation ended with failure ******** {:?}\n", details.as_ref().unwrap_or(&empty_string));
                    navigation_success.set(false).unwrap();
                    break;
                }
                _ => {
                    state_sender.send(None).await?;
                    warn!("unknown");
                }
            }
        }

        info!("\nWaiting for 30 seconds (cool off period) before restarting the connection...\n\n");
        drop(inbound);
        drop(state_sender);
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}
