use std::collections::HashSet;

pub fn part_01(data: &str) -> usize {
    data.lines()
        .map(|line| {
            let (row_raw, col_raw) = (&line[0..7], &line[7..]);
            //println!("{} --> {} - {}", line, row_raw, col_raw);
            let (row, col) = (parse_to_binary(row_raw, 'B'), parse_to_binary(col_raw, 'R'));
            let seat_id = row * 8 + col;
            //println!("{} --> {} - {} --> {}", line, row, col, seat_id);
            seat_id
        })
        .max()
        .unwrap()
}

fn parse_to_binary(data: &str, bin_char: char) -> usize {
    data.chars().rev().enumerate().fold(0, |init, (idx, c)| {
        if c == bin_char {
            return init + 2_usize.pow(idx as u32);
        }
        init
    })
}

#[cfg(test)]
mod test_part_01 {
    use super::*;

    #[test]
    fn test_parsing() {
        // row
        assert_eq!(parse_to_binary("FBFBBFF", 'B'), 44);

        // col
        assert_eq!(parse_to_binary("RLR", 'R'), 5);
    }

    #[test]
    fn test_seat_id() {
        assert_eq!(70 * 8 + 7, 567);
    }
}

/** PART 2 **/

pub fn part_02(data: &str) -> usize {
    let my_seat_ids: HashSet<usize> = data.lines()
        .map(|line| {
            let (row_raw, col_raw) = (&line[0..7], &line[7..]);
            //println!("{} --> {} - {}", line, row_raw, col_raw);
            let (row, col) = (parse_to_binary(row_raw, 'B'), parse_to_binary(col_raw, 'R'));
            let seat_id = row * 8 + col;
            //println!("{} --> {} - {} --> {}", line, row, col, seat_id);
            seat_id
        }).collect();
    
    let mut possible_seat_ids = HashSet::with_capacity(128*8);
    for row in 0..=127 {
        for col in 0..=7 {
            possible_seat_ids.insert(row * 8 + col);
        }
    }
    let diff= possible_seat_ids.difference(&my_seat_ids);
    let mut result = diff.into_iter().collect::< Vec<_> >();
    result.sort();
    
    println!("{:?}", result);
    0
}
