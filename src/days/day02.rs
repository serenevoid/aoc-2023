use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("./input/02.txt").expect("Unable to read input");
    let sol1: u64 = part_1(&input);
    let sol2: u64 = part_2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part_1(input: &str) -> u64 {
    let mut sum: u64 = 0;
    let margin_count = [12, 13, 14];
    for line in input.lines() {
        let game_data = line
            .split(":")
            .collect::<Vec<&str>>();
        let game_number = game_data[0][5..]
            .parse::<u64>()
            .expect("Cannot parse game index");
        let mut is_cube_count_contained = true;
        for subgame in game_data[1].split(";") {
            for cube in subgame.split(",") {
                for (index, cube_color)in ["red", "green", "blue"].iter().enumerate() {
                    if cube.trim().contains(cube_color) {
                        let cube_count = cube
                            .trim()
                            .split_whitespace()
                            .collect::<Vec<&str>>()[0]
                            .parse::<u64>()
                            .expect("Cannot parse cube count");
                        if cube_count > margin_count[index] {
                            is_cube_count_contained = false;
                            break;
                        }
                    }
                }
            }
        }
        if is_cube_count_contained {
            sum += game_number;
        }
    }
    sum
}

fn part_2(input: &str) -> u64 {
    let mut sum: u64 = 0;
    for line in input.lines() {
        let game_data = line
            .split(":")
            .collect::<Vec<&str>>();
        let mut minimum_counts = [0, 0, 0];
        for subgame in game_data[1].split(";") {
            for cube in subgame.split(",") {
                for (index, cube_color)in ["red", "green", "blue"].iter().enumerate() {
                    if cube.trim().contains(cube_color) {
                        let cube_count = cube
                            .trim()
                            .split_whitespace()
                            .collect::<Vec<&str>>()[0]
                            .parse::<u64>()
                            .expect("Cannot parse cube count");
                        if minimum_counts[index] < cube_count {
                            minimum_counts[index] = cube_count;
                        }
                    }
                }
            }
        }
        sum += minimum_counts[0] * minimum_counts[1] * minimum_counts[2];
    }
    sum
}
