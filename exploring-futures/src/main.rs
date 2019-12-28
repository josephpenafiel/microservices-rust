#![allow(dead_code)]
mod channels;
use channels::{multiple, single, alt_udp_echo, send_spawn};


fn main() {
    single();
    multiple();
    send_spawn();
    println!("starting UDP socket!");
    alt_udp_echo().unwrap();
}

