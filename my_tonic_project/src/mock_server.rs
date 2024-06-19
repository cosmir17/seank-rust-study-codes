use std::{error::Error, pin::Pin};
use std::time::Duration;
use tokio::time;
use tokio_stream::{Stream, StreamExt};

use tonic::{Request, Response, Status, Streaming};
use tonic::transport::Server;

use proto::{ClientMsg, ServerMsg};
use proto::controller_server::Controller;
use crate::proto::server_msg::{Data, SimOver, SimStart, SimUpdate};
use crate::proto::{Area, Point};


pub mod proto {
    tonic::include_proto!("service");
}

type ResponseStream = Pin<Box<dyn Stream<Item=Result<ServerMsg, Status>> + Send + 'static>>;

fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
    let mut err: &(dyn Error + 'static) = err_status;

    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }

        // h2::Error do not expose std::io::Error with `source()`
        // https://github.com/hyperium/h2/pull/462
        if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
            if let Some(io_err) = h2_err.get_io() {
                return Some(io_err);
            }
        }

        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}


static SERVER_START_MSG: ServerMsg = ServerMsg {
    data: Some(Data::Start(SimStart {
        l: Some(Point { x: 0.0 }),
        limit: Some(Area { n_point: Some(Point { x: -50.0 }), m_point: Some(Point { x: 50.0 }) }),
        g: Some(Area { n_point: Some(Point { x: -26.0 }), m_point: Some(Point { x: -24.0 }) }),
    }))
};


#[derive(Debug)]
pub struct MockRPCServer {}

#[tonic::async_trait]
impl Controller for MockRPCServer {
    type ConnectionStream = ResponseStream;

    async fn connection(&self, request: Request<Streaming<ClientMsg>>) -> Result<Response<Self::ConnectionStream>, Status> {
        let mut stream = request.into_inner();
        let mut count = 0;
        let mut interval = time::interval(Duration::from_secs_f32(0.02));
        let output = async_stream::try_stream! {

            // todo this doesn't work
            // while let Some(client_msg) = stream.next().await {
            //     match client_msg {
            //         Ok(message) => {
            //             println!("started 4");
            //             yield SERVER_START_MSG.clone()
            //         },
            //         Err(e) => {
            //             println!("started 5");
            //             yield SERVER_START_MSG.clone()
            //         }
            //     }
            // }

            loop {
                interval.tick().await;

                if count == 0 {
                    yield SERVER_START_MSG.clone()
                } else if count > 300 {
                    yield ServerMsg {
                        data: Some(Data::Ended(SimOver { success: true, details: Some("success".to_owned()), }))
                    }
                } else {
                    yield ServerMsg {
                        data: Some(Data::Update(SimUpdate {
                            l: Some(Point { x: -27.108433 }),
                        }))
                    }
                }

                count += 1;
            }
        };

        Ok(Response::new(
            Box::pin(output) as Self::ConnectionStream
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = MockRPCServer {};

    Server::builder()
        .add_service(proto::controller_server::ControllerServer::new(server))
        .serve("[::1]:50051".parse().unwrap())
        .await?;

    Ok(())
}

