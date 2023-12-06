use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, collections::HashMap};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("./input/04.txt").expect("Cannot read from file");
    let sol1: u64 = part_1(&input);
    let sol2: u64 = part_2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part_1(input: &str) -> u64 {
    let sums = input
        .split("\n")
        .map(|line| {
            if !line.is_empty() {
                let data = line
                    .split(":")
                    .collect::<Vec<&str>>()[1]
                    .split("|")
                    .collect::<Vec<&str>>();
                let winning_values = data[0]
                    .split_whitespace()
                    .map(|value| {
                        value.parse::<u64>().expect("Cannot parse number")
                    })
                    .collect::<Vec<u64>>();
                let received_values = data[1]
                    .split_whitespace()
                    .map(|value| {
                        value.parse::<u64>().expect("Cannot parse number")
                    })
                    .collect::<Vec<u64>>();
                let mut point = 0;
                for value in received_values {
                    if winning_values.contains(&value) {
                        if point == 0 {
                            point = 1;
                        } else {
                            point *= 2;
                        }
                    }
                }
                point
            } else {
                0
            }
        })
        .collect::<Vec<u64>>();
    let mut sum = 0;
    for value in sums {
        sum += value;
    }
    sum
}

fn part_2(input: &str) -> u64 {
    let mut map = HashMap::new();
    input
        .split("\n")
        .map(|line| {
            if !line.is_empty() {
                let cardnumber = line
                    .split(":")
                    .collect::<Vec<&str>>()[0]
                    .split_whitespace()
                    .collect::<Vec<&str>>()[1]
                    .parse::<u64>()
                    .expect("Cannot parse card number");
                let data = line
                    .split(":")
                    .collect::<Vec<&str>>()[1]
                    .split("|")
                    .collect::<Vec<&str>>();
                let winning_values = data[0]
                    .split_whitespace()
                    .map(|value| {
                        value.parse::<u64>().expect("Cannot parse data number")
                    })
                    .collect::<Vec<u64>>();
                let received_values = data[1]
                    .split_whitespace()
                    .map(|value| {
                        value.parse::<u64>().expect("Cannot parse data number")
                    })
                    .collect::<Vec<u64>>();
                let mut count = cardnumber;
                for value in received_values {
                    if winning_values.contains(&value) {
                        count += 1;
                        match map.get_mut(&count) {
                            Some(x) => {
                                *x += 1;
                            },
                            None => {
                                map.insert(cardnumber, 1);
                            },
                        }
                    }
                }
                0
            } else {
                0
            }
        });
}
