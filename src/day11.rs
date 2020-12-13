use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum State {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for State {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => {
                eprintln!("ERROR: {}", c);
                panic!("lol");
            },
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Floor => write!(f, "."),
            Self::Empty => write!(f, "L"),
            Self::Occupied => write!(f, "#")
        }
    }
}

#[derive(Debug)]
struct SeatingArea {
    seats: HashMap<(isize, isize), State>,
}

impl From<&str> for SeatingArea {
    fn from(data: &str) -> Self {
        use std::convert::TryFrom;

        let mut cells = HashMap::new();
        data
        .trim()
        .lines()
        .enumerate()
        .for_each(|(idx_i, line)| {
            let idx_i = isize::try_from(idx_i).unwrap();
            line
            .trim()
            .chars()
            .enumerate()
            .for_each(|(idx_j, c)| {
                let idx_j = isize::try_from(idx_j).unwrap();
                println!("Inserting at ({:>2}, {:>2}): {:?}", idx_i, idx_j, State::from(c));
                cells.insert((idx_i, idx_j), State::from(c));
            })
        });

        SeatingArea{
            seats: cells,
        }
    }
}

impl From<String> for SeatingArea {
    fn from(data: String) -> Self {
        let a: &str = &data;
        Self::from(a)
    }
}

impl Display for SeatingArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.seats.iter().fold(write!(f, ""),|_, v| write!(f, "{} {} {}\n", v.0.0, v.0.1, v.1))
    }
}

impl SeatingArea {
    fn get(&self, i: isize, j: isize) -> State {
        match self.seats.get(&(i, j)) {
            None => State::Floor,
            Some(&s) => s,
        }
    }

    fn count_occ(&self) -> usize {
        self.seats.clone().iter().filter(|(_, &state)| state == State::Occupied).count()
    }

    fn count_empty(&self) -> usize {
        self.seats.iter().filter(|(_, &state)| state == State::Empty).count()
    }

    fn count_floor(&self) -> usize {
        self.seats.iter().filter(|(_, &state)| state == State::Floor).count()
    }

    fn occupied_neighbours(&self, x: isize, y: isize) -> isize {
        let mut occupied_seats = 0;
        for i in x-1..=x+1 {
            for j in y-1..=y+1 {
                if i == x && j ==y {
                    continue;
                }
                match self.get(i, j) {
                    State::Floor => {}
                    State::Empty => {}
                    State::Occupied => {occupied_seats += 1}
                }
            }
        }
        occupied_seats
    }

    fn occupied_visible(&self, x: isize, y: isize) -> isize {
        let mut occupied_seats = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                let mut i = i;
                let mut j = j; 
                //println!("### ({:>2}, {:>2}) ### {:?}", x+i, y+j, self.seats.get(&(x+y, y+j)));
                if i == 0 && j == 0 { continue };

                while let Some(state) = self.seats.get(&(x + i, y + j)) {
                    let mut should_break = false;
                    match *state {
                        State::Floor => {}
                        State::Empty => {
                            //println!("Found free seat at {:>2}, {:>2}", x+i, y+j);
                            should_break = true
                        }
                        State::Occupied => {
                            //println!("Found occu seat at {:>2}, {:>2}", x+i, y+j);
                            occupied_seats += 1;
                            should_break = true
                        }
                    }
                    if should_break {
                        break;
                    }
                    //println!("BEFORE: {:>3} - {:>3}", i, j);
                    //println!("BEFORE: {:>3} - {:>3}", x+i, y+j);
                    if      i <  0 && j <  0 {i -= 1; j -= 1}
                    else if i <  0 && j == 0 {i -= 1        }
                    else if i <  0 && j >  0 {i -= 1; j += 1}
                    else if i == 0 && j <  0 {        j -= 1}
                    else if i == 0 && j == 0 {eprintln!("FUCK")}
                    else if i == 0 && j >  0 {        j += 1}
                    else if i >  0 && j <  0 {i += 1; j -= 1}
                    else if i >  0 && j == 0 {i += 1;       }
                    else if i >  0 && j >  0 {i += 1; j += 1}
                    //println!(" AFTER: {:>3} - {:>3}", i, j);
                    //println!(" AFTER: {:>3} - {:>3}\n", x+i, y+j);
                }
                //println!();
            }
        }

        occupied_seats
    }

    fn next_state_part_1(&mut self) -> usize {
        let mut new_seats = self.seats.clone();
        let mut changes = 0;

        for (&idx, &seat_state) in &self.seats {
            let neighs = self.occupied_neighbours(idx.0, idx.1);
            if seat_state == State::Empty && neighs == 0 {
                new_seats.insert((idx.0, idx.1), State::Occupied);
                changes += 1;
                continue;
            }
            if seat_state == State::Occupied && neighs >= 4 {
                new_seats.insert((idx.0, idx.1), State::Empty);
                changes += 1;
                continue
            }
            if seat_state == State::Floor {
                new_seats.insert((idx.0, idx.1), State::Floor);
                continue;
            }
        }

        if changes >= 1 {
            self.seats = new_seats;
        }
        changes
    }

    fn next_state_part_2(&mut self) -> usize {
        let mut new_seats = self.seats.clone();
        let mut changes = 0;

        for (&idx, &seat_state) in &self.seats {
            let neighs = self.occupied_visible(idx.0, idx.1);
            if seat_state == State::Empty && neighs == 0 {
                new_seats.insert((idx.0, idx.1), State::Occupied);
                changes += 1;
                continue;
            }
            if seat_state == State::Occupied && neighs >= 5 {
                new_seats.insert((idx.0, idx.1), State::Empty);
                changes += 1;
                continue
            }
            if seat_state == State::Floor {
                new_seats.insert((idx.0, idx.1), State::Floor);
                continue;
            }
        }

        if changes >= 1 {
            self.seats = new_seats;
        }
        changes
    }
}

pub fn part_01(data: &str) -> usize {
    let mut seating_area = SeatingArea::from(data);
    let mut changes = 0;

    println!("Occ: {:>4} - Empty: {:>4} - Floor: {:>4} - Changes: {:>4}",
        seating_area.count_occ(), seating_area.count_empty(), seating_area.count_floor(), changes);
    loop {
        changes = seating_area.next_state_part_1();
        println!("Occ: {:>4} - Empty: {:>4} - Floor: {:>4} - Changes: {:>4}",
            seating_area.count_occ(), seating_area.count_empty(), seating_area.count_floor(), changes);
        if changes == 0 {
            break;
        }
    }
    seating_area.count_occ()
}


pub fn part_02(data: &str) -> usize {
    let mut seating_area = SeatingArea::from(data);
    let mut changes = 0;

    let a =seating_area.occupied_visible(4, 3);
    println!("occupied: {:>4}", a);
    seating_area.next_state_part_2();
    println!("occupied: {:>4}", a);

    println!("Occ: {:>4} - Empty: {:>4} - Floor: {:>4} - Changes: {:>4}",
        seating_area.count_occ(), seating_area.count_empty(), seating_area.count_floor(), changes);
    loop {
        changes = seating_area.next_state_part_2();
        println!("Occ: {:>4} - Empty: {:>4} - Floor: {:>4} - Changes: {:>4}",
            seating_area.count_occ(), seating_area.count_empty(), seating_area.count_floor(), changes);
        if changes == 0 {
            break;
        }
    }

    seating_area.count_occ()
}

#[test]
fn test_fuckery() {
    let a = r#"
    .......#.
    ...#.....
    .#.......
    .........
    ..#L....#
    ....#....
    .........
    #........
    ...#....."#;

    let mut seating_area = SeatingArea::from(a);
    let mut changes = 0;
    //assert_eq!(seating_area.occupied_visible(0, 7), 0);
    assert_eq!(seating_area.occupied_visible(0, 0), 2);
    //assert_eq!(seating_area.occupied_visible(4, 3), 8);

    println!("Occ: {:>4} - Empty: {:>4} - Floor: {:>4} - Changes: {:>4}",
        seating_area.count_occ(), seating_area.count_empty(), seating_area.count_floor(), changes);
    loop {
        changes = seating_area.next_state_part_2();
        println!("Occ: {:>4} - Empty: {:>4} - Floor: {:>4} - Changes: {:>4}",
            seating_area.count_occ(), seating_area.count_empty(), seating_area.count_floor(), changes);
        if changes == 0 {
            break;
        }
    }

    println!("Occupied: {}", seating_area.count_occ());
}