use std::str;

use aoc2020::day11::*;

fn main() {
    let content = str::from_utf8(include_bytes!("./input.txt")).unwrap();
    println!("\nRESULT PART 1: {:?}\n", part_01(content));
    println!("RESULT PART 2: {:?}", part_02(content));
}
