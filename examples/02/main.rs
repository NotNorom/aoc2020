use std::fs::File;

use aoc2020::{day02::*, read_stream_and_reset};

fn main() {
    let content = read_stream_and_reset(&mut File::open("./examples/02/input.txt").unwrap());
    println!("{:}", part_01(&content));

    println!("{:}", part_02(&content));
}