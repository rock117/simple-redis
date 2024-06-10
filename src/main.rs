use crate::error::RedisError;
use crate::resp::Resp::{Nulls, SimpleStrings};
use crate::resp::{Resp, RespCodec};
use futures::SinkExt;
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};
use tracing::level_filters::LevelFilter;
use tracing::{info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, EnvFilter, Registry};

mod command;
mod datatype;
mod error;
mod executor;
mod resp;
mod storage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_log_context()?;

    let addr = "0.0.0.0:16379";
    info!("Simple-Redis-Server is listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from: {}", raddr);
        tokio::spawn(async move {
            match handle_stream(stream).await {
                Ok(_) => {
                    info!("Connection from {} exited", raddr);
                }
                Err(e) => {
                    warn!("handle error for {}: {:?}", raddr, e);
                }
            }
        });
    }
}

async fn handle_stream(mut stream: TcpStream) -> Result<(), RedisError> {
    let mut framed = Framed::new(stream, RespCodec);
    loop {
        match framed.next().await {
            Some(Ok(Resp::BulkStrings(bulk_string))) => {
                info!("Received frame: {:?}", bulk_string);
            }
            Some(Ok(frame)) => {
                warn!("invalid command args: {:?}", frame);
                framed.send(Nulls).await?; // TODO
            }
            Some(Err(e)) => todo!(),
            None => return Ok(()),
        }
    }
    todo!()
}

fn init_log_context() -> Result<(), Box<dyn Error>> {
    let filter = EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into());
    let subscriber = Registry::default()
        .with(
            fmt::layer()
                .compact()
                .with_ansi(true)
                .with_file(false)
                .with_line_number(false)
                .with_thread_ids(false)
                .with_target(false),
        )
        .with(filter);
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
