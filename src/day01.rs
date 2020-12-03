pub fn part_01_slow(data: String) -> i32 {
    let numbers: Vec<i32> = data
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    for x in numbers.clone() {
        for y in numbers.clone() {
            for z in numbers.clone() {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    0
}

pub fn part_01_perm_01(data: String) -> i64 {
    let numbers: Vec<_> = data
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut result = 0;
    permutator::large_combination(&numbers, 3, |x| {
        if x.iter().fold(0, |init, &&x| init + x) == 2020 {
            result = x.iter().fold(1, |init, &&x| init * x);
        }
    });

    result
}

pub fn part_01(data: String) -> u64 {
    use permutator::copy::LargeCombinationIterator;

    let numbers: Vec<_> = data
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut combinations = LargeCombinationIterator::new(&numbers, 3);

    let combination: Option<Vec<u64>> = combinations.find(|comb| comb.iter().fold(0u64, |init, x| init + x) == 2020);

    combination.unwrap().iter().fold(1, |init, &x| init * x)
}