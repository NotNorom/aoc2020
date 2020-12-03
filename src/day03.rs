use std::ops::Index;

struct Map {
    height: usize,
    width: usize,
    data: Vec<Vec<char>>,
}

impl Map {
    fn count_trees(&self, down: usize, right: usize) -> usize {
        let mut count = 0;

        {
            let mut w = 0;
            for h in (0..self.height).step_by(down) {
                if self[[h, w]] == '#' {
                    count += 1;
                }
                w += right;
            }
        }

        count
    }
}

impl Index<[usize; 2]> for Map {
    type Output = char;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.data[index[0]][index[1] % self.width]
    }
}

impl From<&str> for Map {
    fn from(data: &str) -> Self {
        let m: Vec<Vec<char>> = data
            .split_whitespace()
            .map(|row| row.chars().collect())
            .collect();

        Map {
            height: m.len(),
            width: m[0].len(),
            data: m,
        }
    }
}

pub fn part_01(input: &str) -> usize {
    let map = Map::from(input);
    map.count_trees(1, 3)
}

pub fn part_02(input: &str) -> usize {
    let map = Map::from(input);
    let directions = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    let mut result = 1;
    for dir in directions {
        let count = map.count_trees(dir.0, dir.1);
        result *= count;
    }

    result
}
