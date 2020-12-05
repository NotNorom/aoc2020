static _OPTIONAL: &str = "cid";
static NEEDED: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub fn part_01(data: &str) -> usize {
    data.split_terminator("\n\n")
        .filter(|&p| is_passport_valid_part_1(p))
        .count()
}

fn is_passport_valid_part_1(passport: &str) -> bool {
    NEEDED.iter().all(|&n| passport.contains(n))
}

#[test]
fn test_passport_vailidity_part_1() {
    assert_eq!(is_passport_valid_part_1("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"), true);
    assert_eq!(is_passport_valid_part_1("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929"), false);
    assert_eq!(is_passport_valid_part_1("hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm"), true);
    assert_eq!(is_passport_valid_part_1("hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in"), false);
}

/* PART 2 */

pub fn part_02(data: &str) -> usize {
    data.split_terminator("\n\n").filter(|&p| is_passport_valid_part_2(p)).count()
}

fn is_passport_valid_part_2(passport: &str) -> bool {
    if NEEDED.iter().any(|&n| !passport.contains(n)) {
        return false;
    }
    passport.split_whitespace().all(|field| {
        let temp: Vec<&str> = field.split_terminator(':').collect();
        is_field_valid(temp[0], temp[1])
    })
}


fn is_field_valid(name: &str, value: &str) -> bool {
    match name {
        "byr" => {
            let v: usize = value.parse().unwrap();
            1920 <= v && v <= 2002
        }
        "iyr" => {
            let v: usize = value.parse().unwrap();
            2010 <= v && v <= 2020
        }
        "eyr" => {
            let v: usize = value.parse().unwrap();
            2020 <= v && v <= 2030
        }
        "hgt" => {
            match &value[value.len()-2..] {
                "cm" => {
                    let number: usize = (&value[..value.len()-2]).parse().unwrap();
                    150 <= number && number <= 193
                }
                "in" => {
                    let number: usize = (&value[..value.len()-2]).parse().unwrap();
                    59 <= number && number <= 76
                }
                _ => false,
            }
        }
        "hcl" => {
            let hex = "0123456789abcdef";
            let chars: Vec<char> = value.chars().collect();
            if chars[0] != '#' {
                return false;
            }
            !chars.iter().skip(1).any(|&c| !hex.contains(c))
        }
        "ecl" => {
            let colours = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            colours.contains(&value)
        }
        "pid" => {
            if value.len() != 9 {
                return false;
            }
            match value.parse::<usize>() {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        "cid" => true,
        _ => false
    }
}


#[test]
fn test_passport_vailidity_part_2() {
    // invalid
    assert_eq!(is_passport_valid_part_2("eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"), false);
    assert_eq!(is_passport_valid_part_2("iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946"), false);
    assert_eq!(is_passport_valid_part_2("hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"), false);
    assert_eq!(is_passport_valid_part_2("hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007"), false);

    // valid
    assert_eq!(is_passport_valid_part_2("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f"), true);
    assert_eq!(is_passport_valid_part_2("eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"), true);
    assert_eq!(is_passport_valid_part_2("hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022"), true);
    assert_eq!(is_passport_valid_part_2("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"), true);
}