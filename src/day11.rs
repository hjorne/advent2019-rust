use super::intcode::IntcodeComputer;
use super::robot::Robot;
use std::fs;

pub fn day11() {
    let input = fs::read_to_string("inputs/day11.txt").unwrap();
    let handle = IntcodeComputer::new(&input, Vec::new());
    let mut robot = Robot::new(handle);
    robot.run();
}
