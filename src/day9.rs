use super::intcode::IntcodeComputer;
use std::fs;

pub fn day9() {
    let input = fs::read_to_string("inputs/day9.txt").unwrap();

    run(&input, 1);
    run(&input, 2);
}

fn run(input: &str, val: i64) {
    let handle = IntcodeComputer::new(input);
    handle.tx_input.send(val).unwrap();
    handle.thread_handle.join().unwrap();
    println!(
        "Output is {:?}",
        handle.rx_output.try_iter().collect::<Vec<_>>()
    );
}
