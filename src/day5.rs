use std::fs;
use super::intcode::IntcodeComputer;

pub fn day5() {
    let input = fs::read_to_string("inputs/day5.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut cpu = IntcodeComputer::new(&input, vec![1]);
    cpu.run();
    println!("Final output is {}", cpu.output.back().unwrap());
}

fn part2(input: &str) {
    let mut cpu = IntcodeComputer::new(&input, vec![5]);
    cpu.run();
    println!("Final output is {}", cpu.output.back().unwrap());
}
