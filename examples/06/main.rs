use std::str;

use aoc2020::day06::*;

fn main() {
    let content = str::from_utf8(include_bytes!("./input.txt")).unwrap();
    println!("{:}\n", part_01(&content));
    println!("{:}", part_02(&content));
}