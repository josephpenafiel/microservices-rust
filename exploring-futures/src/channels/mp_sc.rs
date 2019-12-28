// ########################################################################
// ################# Multiple Producer Single Consumer ####################
// ########################################################################

use super::*;


pub fn multiple() {
    // multiple producer single consumer

    //  (sender, receiver)
    let (tx_sink, rx_stream) = mpsc::channel::<u8>(8);
    let send1 = tx_sink.clone().send(1); //requires Sink
    let send2 = tx_sink.clone().send(2); // send returns a Sink instance
    let send3 = tx_sink.clone().send(3);
    let receiver = rx_stream
        .fold(0, |acc, value| future::ok(acc + value))
        .map(|x| {
            println!("calculated: {}", x);
        });

    let execute_all = future::join_all(vec![
        to_box(receiver),
        to_box(send1),
        to_box(send2),
        to_box(send3),
    ])
    .map(drop);
    drop(tx_sink); // close the sender
    tokio::run(execute_all);
}
