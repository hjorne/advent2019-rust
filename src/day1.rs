use std::fs;

pub fn day1() {
    let input = fs::read_to_string("inputs/day1.txt").unwrap();
    let input = input.lines().map(|line| line.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let req: i32 = input.iter().map(|&line| requirement(line)).sum();
    println!("Fuel requirements are {}", req);

    let req_recursive: i32 = input.iter().map(|&line| requirement_recursive(line)).sum();
    println!("Fuel requirements are {}", req_recursive);
}

fn requirement(n: i32) -> i32 {
    n / 3 - 2
}

fn requirement_recursive(n: i32) -> i32 {
    let fuel = n / 3 - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + requirement_recursive(fuel)
    }
}
