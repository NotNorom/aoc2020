use std::io::{Read, Seek, SeekFrom};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub fn read_stream_and_reset<T: Read + Seek>(data: &mut T) -> String {
    let mut buffer = String::new();
    data.read_to_string(&mut buffer).unwrap();
    data.seek(SeekFrom::Start(0)).unwrap();
    buffer
}
