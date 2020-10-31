use super::intcode::{IntcodeComputer, ParamMode};
use std::fs;

pub fn day2() {
    let input = fs::read_to_string("inputs/day2.txt").unwrap();
    let cpu = IntcodeComputer::new(&input);
    part1(&cpu);
    part2(&cpu);
}

fn part1(cpu: &IntcodeComputer) {
    let mut cpu = cpu.clone();
    cpu.write(1, 12);
    cpu.write(2, 2);
    cpu.run();
    println!("Value left in position 0 is {}", cpu.read(0, None));
}

fn part2(cpu: &IntcodeComputer) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut cpu = cpu.clone();
            cpu.write(1, noun);
            cpu.write(2, verb);
            cpu.run();
            if cpu.read(0, None) == 19690720 {
                println!("Values for noun/verb is {}", 100 * noun + verb);
                return;
            }
        }
    }
}
