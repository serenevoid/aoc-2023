use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_to_string("input/03.txt").expect("Cannot read input");
    let sol1: u64 = part_1(&input);
    let sol2: u64 = part_2(&input);

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

fn part_2(input: &str) -> u64 {
    let mut sum = 0;
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
        let gears = find_gears(curr);
        sum += get_gear_ratio(gears, prev, curr, next);
    }
    // for last loop
    {
        prev = curr;
        curr = next;
        let gears = find_gears(curr);
        sum += get_gear_ratio(gears, prev, curr, next);
    }
    sum
}

fn find_gears(line: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    for (i, letter) in line.char_indices() {
        if letter == '*' {
            positions.push(i);
        }
    }
    positions
}

fn get_gear_ratio(gears: Vec<usize>, prev: &str, curr: &str, next: &str) -> u64 {
    let mut sum = 0;
    for pos in gears {
        let mut found_first = false;
        let mut is_separated = false;
        let mut found_second = false;
        let mut first_line = "";
        let mut second_line = "";
        let mut first_pos = 0;
        let mut second_pos = 0;
        if !prev.is_empty() {
            for (i, letter) in prev.chars().enumerate() {
                if i <= pos + 1 && i >= pos - 1 {
                    if letter.is_numeric() {
                        if found_first && is_separated {
                            found_second = true;
                            second_line = prev;
                            second_pos = i;
                        } else {
                            found_first = true;
                            first_line = prev;
                            first_pos = i;
                        }
                    } else {
                        if found_first {
                            is_separated = true;
                        }
                    }
                }
            }
        }
        if found_first {
            is_separated = true;
        }
        if !curr.is_empty() {
            for (i, letter) in curr.chars().enumerate() {
                if i <= pos + 1 && i >= pos - 1 {
                    if letter.is_numeric() {
                        if found_first && is_separated {
                            found_second = true;
                            second_line = curr;
                            second_pos = i;
                        } else {
                            found_first = true;
                            first_line = curr;
                            first_pos = i;
                        }
                    } else {
                        if found_first {
                            is_separated = true;
                        }
                    }
                }
            }
        }
        if found_first {
            is_separated = true;
        }
        if !next.is_empty() {
            for (i, letter) in next.chars().enumerate() {
                if i <= pos + 1 && i >= pos - 1 {
                    if letter.is_numeric() {
                        if found_first && is_separated {
                            found_second = true;
                            second_line = next;
                            second_pos = i;
                        } else {
                            found_first = true;
                            first_line = next;
                            first_pos = i;
                        }
                    } else {
                        if found_first {
                            is_separated = true;
                        }
                    }
                }
            }
        }
        if found_first && found_second {
            sum += find_numbers_at_pos(first_line, first_pos) * find_numbers_at_pos(second_line, second_pos);
        }
    }
    sum
}

fn find_numbers_at_pos(line: &str, pos: usize) -> u64 {
    let mut num: u64 = 0;
    let mut is_found = false;
    for (i, letter) in line.char_indices() {
        if i == pos {
            is_found = true;
        }
        if letter.is_numeric() {
            num = (num * 10) + letter.to_digit(10).expect("cannot parse character to number") as u64;
        } else {
            if num != 0 && is_found {
                break;
            }
            num = 0;
        }
    }
    num
}

