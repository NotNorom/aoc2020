fn parse_input(data: &str) -> (isize, Vec<isize>) {
    let mut it = data.lines();
    let timestamp: isize = it.next().unwrap().parse().unwrap_or_default();
    let buses: Vec<isize> = it.next().unwrap().split_terminator(",").filter_map(|s| s.parse().ok()).collect();
    (timestamp, buses)
}

#[test]
fn test_input_parsing() {
    let input = "939
7,13,x,x,59,x,31,19";
    let result = parse_input(&input);
    assert_eq!(result, (939, vec![7isize, 13, 59, 31, 19]));
}

fn get_min_timestamp_and_bus(timestamp: isize, buses: Vec<isize>) -> (isize, isize) {
    buses.iter().map(|&bus| {
        (((timestamp + bus) / bus) * bus, bus)
    }).min().unwrap_or_default()
}

#[test]
fn test_min_timestamp_and_bus() {
    let (timestamp, buses) = (939isize, vec![7isize, 13, 59, 31, 19]);
    let (res_timestamp, res_bus) = get_min_timestamp_and_bus(timestamp, buses);

    assert_eq!(res_timestamp, 944);
    assert_eq!(res_bus, 59);
    assert_eq!((res_timestamp - timestamp) * res_bus, 295);
}

pub fn part_01(data: &str) -> isize {
    let (timestamp, buses) = parse_input(data);
    let (res_timestamp, res_bus) = get_min_timestamp_and_bus(timestamp, buses);
    (res_timestamp - timestamp) * res_bus
}