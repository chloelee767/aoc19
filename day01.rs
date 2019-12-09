use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main () {
    let f = File::open("day01.txt").unwrap();
    let f = BufReader::new(f);

    let mut total_fuel = 0;
    for line in f.lines() {
        let weight: i32 = line.unwrap().parse().unwrap();
        total_fuel += part2_recur(&weight);
    }

    println!("{}", total_fuel);
}

fn part1(mass : &i32) -> i32 {
    return (mass / 3) - 2; // integer division
}

fn part2_recur(mass : &i32) -> i32 {
    let fuel = part1(mass);
    if fuel <= 0 {
        return 0;
    } else {
        return fuel + part2_recur(&fuel);
    }
}

fn part2_iter(mass : &i32) -> i32 {
    let mut total : i32 = 0;
    let mut current : i32 = part1(mass);
    while current > 0 {
        total += current;
        current = part1(&current);
    }
    return total;
}
