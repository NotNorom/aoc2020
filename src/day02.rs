fn is_valid_part_01(input: &str) -> bool {
    let data: Vec<&str> = input
        .split_whitespace()
        .collect();
    let range: Vec<usize> = data[0]
        .split('-')
        .map(|s| s.parse().unwrap())
        .collect();
    let character = &data[1][0..1];
    let count = data[2]
        .chars()
        .filter(|x| x.to_string() == character)
        .count();

    range[0] <= count && count <= range[1]
}

pub fn part_01(input: &str) -> usize {
    let mut count = 0;
    for line in input.split('\n') {
        if line.len() == 0 {
            continue;
        }
        if is_valid_part_01(line) {
            count += 1;
        }
    }
    count
}


fn is_valid_part_02(input: &str) -> bool {
    let data: Vec<&str> = input
        .split_whitespace()
        .collect();
    let positions: Vec<usize> = data[0]
        .split('-')
        .map(|s| s.parse().unwrap())
        .collect();
    let character = &data[1][0..1];
    (data[2]
        .chars()
        .nth(positions[0]-1).unwrap()
        .to_string() == character)
    ^ (data[2]
        .chars()
        .nth(positions[1]-1).unwrap()
        .to_string() == character)
}


pub fn part_02(input: &str) -> usize {
    let mut count = 0;
    for line in input.split('\n') {
        if line.len() == 0 {
            continue;
        }
        if is_valid_part_02(line) {
            count += 1;
        }
    }
    count
}
