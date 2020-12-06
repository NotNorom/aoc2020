pub fn part_01(data: &str) -> usize {
    use std::collections::HashSet;

    data
        .split_terminator("\n\n")
        .map(|v| v.replace("\n", ""))
        .map(|v| v
            .chars()
            .collect::<HashSet<char>>().len())
        .sum()
}

pub fn part_02(data: &str) -> usize {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    data
        .split_terminator("\n\n")
        .map(|v| v
            .split_whitespace()
            .map(|w| w
                .chars()
                .collect::<HashSet<char>>())
            .fold(HashSet::<char>::from_iter("abcdefghijklmnopqrstuvwxyz".chars()),
                |init, mate| init
                    .intersection(&mate)
                    .cloned()
                    .collect::<HashSet<_>>()))
        .map(|v| v.len())
        .sum()
}