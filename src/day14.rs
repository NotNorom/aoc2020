use bit_vec::BitVec;
use std::convert::TryInto;


fn apply_mask(number: u64, mask: &str) -> u64 {
    let mut bits = BitVec::from_bytes(&number.to_be_bytes());

    for (idx, char) in mask.chars().rev().enumerate() {
        match char {
            '1' => {bits.set(bits.len() - idx - 1, true)}
            '0' => {bits.set(bits.len() - idx - 1, false)}
            _ => {}
        }
    }

    let bytes = bits.to_bytes();
    let (int_bytes, rest) = bytes.split_at(std::mem::size_of::<u64>());

    u64::from_be_bytes(int_bytes.try_into().unwrap())
}

#[cfg(test)]
mod test_part_1 {
    use super::*;

    #[test]
    fn test_apply_mask_all_zero() {
        let value: u64    = 0b000000000000000000000000000000001011;
        let mask: &str    =  "000000000000000000000000000000000000";
        let expected: u64 = 0b000000000000000000000000000000000000;
        let result = apply_mask(value, mask);


        eprintln!("EXPECT: {:#64b}\nRESULT: {:#64b}", expected, result);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_apply_mask_all_one() {
        let value: u64    = 0b000000000000000000000000000000001011;
        let mask: &str    =  "111111111111111111111111111111111111";
        let expected: u64 = 0b111111111111111111111111111111111111;
        let result = apply_mask(value, mask);


        eprintln!("EXPECT: {:#64b}\nRESULT: {:#64b}", expected, result);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_apply_mask() {
        let value: u64    = 0b000000000000000000000000000000001011;
        let mask: &str    =  "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let expected: u64 = 0b000000000000000000000000000001001001;
        let result = apply_mask(value, mask);


        eprintln!("EXPECT: {:#64b}\nRESULT: {:#64b}", expected, result);
        assert_eq!(result, expected)
    }
}


pub fn part_01(data: &str) -> u64 {
    0
}