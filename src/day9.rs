use super::intcode::IntcodeComputer;
use num_bigint::BigInt;
use std::fs;
use std::sync::{mpsc, Arc, Mutex};

pub fn day9() {
    let input = fs::read_to_string("inputs/day9.txt").unwrap();

    let (tx_input, rx_input) = mpsc::channel();
    let (tx_output, rx_output) = mpsc::channel();

    let handle = IntcodeComputer::new(
        input.to_owned(),
        Arc::new(Mutex::new(rx_input)),
        Arc::new(Mutex::new(tx_output)),
    );
    tx_input.send(BigInt::from(2)).unwrap();
    handle.join().unwrap();
    println!("Output is {:?}", rx_output.try_iter().collect::<Vec<_>>());
}
