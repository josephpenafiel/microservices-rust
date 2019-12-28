use super::*;

pub fn send_spawn() {
    let (tx_sink, rx_stream) = mpsc::channel::<u8>(8);
    let receiver = rx_stream
        .fold(0, |acc, value| {
            println!("Received: {}", value);
            future::ok(acc + value)
        })
        .map(drop);
    let spawner = stream::iter_ok::<_, ()>(1u8..11u8)
        .map(move |x| {
            let fut = tx_sink.clone().send(x).map(drop).map_err(drop);
            tokio::spawn(fut);
        })
        .collect();
    let executor = future::join_all(vec![to_box(spawner), to_box(receiver)]).map(drop);
    tokio::run(executor);
}
