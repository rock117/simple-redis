use crate::command;
use crate::context::AppContext;
use crate::error::RedisError;
use crate::resp::{nulls, simple_errors, SimpleStrings};
use crate::resp::frame::RespCodec;
use futures::SinkExt;
use std::io;
use tokio::io::Interest;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};
use tracing::{info, warn};
use crate::resp::Resp;

pub async fn handle_stream(mut stream: TcpStream) -> Result<(), RedisError> {
    let mut framed = Framed::new(stream, RespCodec); // RespCodec
    loop {
        match framed.next().await {
            // Some(Ok(Resp::BulkStrings(bulk_strings))) => {
            //     info!("Received frame: {}", bulk_strings);
            //     let cmd = command::parse_command(bulk_strings);
            //     if let Some(cmd) = cmd {
            //         let resp = cmd.execute(&AppContext)?;
            //         framed.send(resp).await?;
            //     }
            // }
            Some(Ok(frame)) => {
                warn!("get command args: {:?}", frame);
                framed
                    .send(Resp::simple_strings("OK".into()))
                    .await?;
            }
            Some(Err(e)) => {
                warn!("Failed to handle frame: {:?}", e);
                framed
                    .send(Resp::simple_errors("Error".into()))
                    .await?;
                return Ok(());
            }
            None => return Ok(()), // no data available to read
        }
    }
}

fn is_fatal_error(error: &RedisError) -> bool {
    match error {
        RedisError::IOError(_) => true,
        _ => false,
    }
}
