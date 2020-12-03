use std::fs::File;

use aoc2020::{day03::*, read_stream_and_reset};

fn main() {
    let content = read_stream_and_reset(&mut File::open("./examples/03/input.txt").unwrap());
    println!("{:}\n", part_01(&content));
    println!("{:}", part_02(&content));
}