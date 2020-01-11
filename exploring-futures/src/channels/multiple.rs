// ########################################################################
// ########### Using channels to use Sink in multiple places ##############
// ########################################################################
use failure::Error;
use tokio::net::{UdpSocket, UdpFramed};
use tokio::codec::LinesCodec;
use super::*;

fn other<E>(err: E) -> std::io::Error
where E: Into<Box<dyn std::error::Error + Send + Sync>> {
    std::io::Error::new(std::io::ErrorKind::Other, err)
}

pub fn alt_udp_echo() -> Result<(), Error> {
    let from = "0.0.0.0:12345".parse()?;
    let socket = UdpSocket::bind(&from)?;
    let framed = UdpFramed::new(socket, LinesCodec::new());
    let (sink, stream) = framed.split();
    let (tx, rx) = mpsc::channel(16);
    let rx = rx.map_err(|_| other("can't take a message"))
        .fold(sink, |sink, frame| {
            println!("receiving...");
            sink.send(frame)
        });
    let process = stream.and_then(move |args| {
        tx.clone()
            .send(args)
            .map(drop)
            .map_err(other)
    }).collect();
    let executor = future::join_all(vec![
        to_box(rx),
        to_box(process),
    ]).map(drop);
    Ok(tokio::run(executor))
}