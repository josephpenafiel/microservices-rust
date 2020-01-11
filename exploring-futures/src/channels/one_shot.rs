// ########################################################################
// ################### One-Shot (single-msg channel) ######################
// ########################################################################

use super::*;

pub fn single() {
    let (tx, rx_future) = oneshot::channel::<u8>();
    let receiver = rx_future.map(|x| {
        let new_val = x * 4;
        println!("received: {}, calculated {}", x, new_val);
    });
    let sender = tx.send(10);
    let executor = future::join_all(vec![to_box(receiver), to_box(sender)]).map(drop);

    tokio::run(executor);
}
