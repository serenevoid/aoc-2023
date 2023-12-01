use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("./input/01_01.txt").expect("Cannot parse input 1");
    let sol1: u64 = part_1(&input);
    let sol2: u64 = part_2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part_1(input: &str) -> u64 {
    let mut sum: u64 = 0;
    for line in input.lines() {
        let mut first_digit: u64 = 0;
        let mut last_digit: u64 = 0;
        for char in line.chars() {
            if char.is_digit(10) {
                if first_digit == 0 {
                    first_digit = u64::from(char.to_digit(10).expect("Invalid number"));
                } else {
                    last_digit = u64::from(char.to_digit(10).expect("Invalid number"));
                }
            }
        }
        if last_digit == 0 {
            last_digit = first_digit;
        }
        sum += 10 * first_digit + last_digit;
    }
    sum
}

#[derive(Debug)]
struct ValueLocation {
    value: u64,
    index: usize,
}

fn part_2(input: &str) -> u64 {
    let mut sum: u64 = 0;
    let numbers_string = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for line in input.lines() {
        let mut first_digit = ValueLocation { value: 0, index: 0 };
        let mut last_digit = ValueLocation { value: 0, index: 0 };
        for (index, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                if first_digit.value == 0 {
                    first_digit.value = u64::from(char.to_digit(10).expect("Invalid number"));
                    first_digit.index = index;
                } else {
                    last_digit.value = u64::from(char.to_digit(10).expect("Invalid number"));
                    last_digit.index = index;
                }
            }
        }
        if last_digit.value == 0 {
            last_digit = ValueLocation { ..first_digit };
        }
        for (index, number) in numbers_string.iter().enumerate() {
            let matches = line.match_indices(number);
            for (i, _) in matches {
                if i < first_digit.index {
                    first_digit.value = (index + 1) as u64;
                    first_digit.index = i;
                } else if i > last_digit.index {
                    last_digit.value = (index + 1) as u64;
                    last_digit.index = i;
                }
            }
        }
        sum += 10 * first_digit.value + last_digit.value;
    }
    sum
}
