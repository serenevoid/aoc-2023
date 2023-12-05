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
    let mut sum: u64 = 0;
    let mut prev = "";
    let mut curr = "";
    let mut next = "";
    for (_, line) in input.lines().enumerate() {
        if next.is_empty() {
            next = line;
            continue;
        }
        if !curr.is_empty() {
            prev = curr;
        }
        curr = next;
        next = line;
        let numbers = find_numbers(curr);
        sum += get_part_sum(numbers, prev, curr, next);
    }
    // for last loop
    {
        prev = curr;
        curr = next;
        let numbers = find_numbers(curr);
        sum += get_part_sum(numbers, prev, curr, next);
    }
    sum
}

fn find_numbers(line: &str) -> Vec<(usize, u64)> {
    let mut values = Vec::new();
    let mut num: u64 = 0;
    let mut pos = 0;
    for (i, letter) in line.char_indices() {
        if letter.is_numeric() {
            if num == 0 {
                pos = i;
            }
            num = (num * 10) + letter.to_digit(10).expect("cannot parse character to number") as u64;
        } else {
            if num != 0 {
                values.push((pos, num));
            }
            num = 0;
        }
    }
    // For values that come at the end
    if num != 0 {
        values.push((pos, num));
    }
    values
}

fn get_part_sum(nums: Vec<(usize, u64)>, prev: &str, curr: &str, next: &str) -> u64 {
    let mut sum = 0;
    for (pos, num) in nums {
        let num_len = num.checked_ilog10().unwrap_or(0) + 1;
        let window_width = num_len as usize + 1;
        let mut is_part_id = false;
        let init_pos = if pos != 0 { pos - 1 } else { pos };
        let final_pos = if (pos + window_width ) >= curr.chars().count() {
            pos + window_width 
        } else {
            pos + window_width - 1
        };
        if !prev.is_empty() {
            for (i, letter) in prev.chars().enumerate() {
                if i >= init_pos && i <= final_pos {
                    if !letter.is_numeric() && letter != '.' {
                        is_part_id = true;
                        break;
                    }
                }
            }
        }
        if !curr.is_empty() && !is_part_id {
            for (i, letter) in curr.chars().enumerate() {
                if i >= init_pos && i <= final_pos {
                    if !letter.is_numeric() && letter != '.' {
                        is_part_id = true;
                    }
                }
            }
        }
        if !next.is_empty() && !is_part_id {
            for (i, letter) in next.chars().enumerate() {
                if i >= init_pos && i <= final_pos {
                    if !letter.is_numeric() && letter != '.' {
                        is_part_id = true;
                    }
                }
            }
        }
        if is_part_id {
            sum += num;
        }
    }
    sum
}
