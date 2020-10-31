use super::intcode::IntcodeComputer;
use permute::permutations_of;
use std::fs;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub fn day7() {
    let input = fs::read_to_string("inputs/day7.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut max = 0;

    permutations_of(&vec![0, 1, 2, 3, 4]).for_each(|permutation| {
        let mut output = 0;
        permutation.for_each(|&phase| {
            let (tx_input, rx_input) = mpsc::channel();
            let (tx_output, rx_output) = mpsc::channel();

            let handle = IntcodeComputer::new(
                input.to_owned(),
                Arc::new(Mutex::new(rx_input)),
                Arc::new(Mutex::new(tx_output)),
            );
            vec![phase, output]
                .into_iter()
                .for_each(|v| tx_input.send(v).unwrap());
            handle.join().unwrap();
            output = rx_output.recv().unwrap();
        });

        if output > max {
            max = output;
        }
    });

    println!("Max output found was {}", max);
}

fn part2(input: &str) {
    let mut max = 0;
    let size = 5usize;

    permutations_of(&(size..size + 5).collect::<Vec<usize>>()).for_each(|permutation| {
        let (txs, rxs): (Vec<_>, Vec<_>) = (0..size)
            .map(|_| {
                let (tx, rx) = mpsc::channel();
                (Arc::new(Mutex::new(tx)), Arc::new(Mutex::new(rx)))
            })
            .unzip();

        let handles = permutation
            .enumerate()
            .map(|(i, &phase)| {
                let input = input.to_owned();

                let (tx, rx) = (txs[i].clone(), rxs[(size + i - 1) % size].clone());
                txs[i].lock().unwrap().send(phase as i64).unwrap();

                IntcodeComputer::new(input.clone(), rx, tx)
            })
            .collect::<Vec<_>>();

        txs[size - 1].lock().unwrap().send(0).unwrap();

        handles
            .into_iter()
            .for_each(|handle| handle.join().unwrap());

        let output = rxs[size - 1].lock().unwrap().recv().unwrap();
        if output > max {
            max = output;
        }
    });
    println!("Max output found was {}", max);
}
