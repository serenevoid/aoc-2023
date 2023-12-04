use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_to_string("input/03.txt").expect("Cannot read input");
    let sol1: u64 = part_1(&input);
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn part_1(input: &str) -> u64 {
    let sum: u64 = 0;
    let mut prev_line = "";
    let mut curr_line = "";
    let mut next_line = "";
    let mut pos = Vec::new();
    for line in input.lines() {
        let mut num: u64 = 0;
        if next_line.is_empty() {
            next_line = line;
            continue;
        }
        if !curr_line.is_empty() {
            prev_line = curr_line;
        }
        if !next_line.is_empty() {
            curr_line = next_line;
        }
        for (i, letter) in curr_line.char_indices() {
            if !letter.is_numeric() && letter != '.' {
                pos.push(i);
            }
        }
        if !prev_line.is_empty() {
            for (i, letter) in prev_line.char_indices() {
                if letter.is_numeric() {
                    num = num * 10 + letter as u64;
                } else {
                    num = 0;
                }
            }
        }
    }
    sum
}
