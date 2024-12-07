use std::{path::Iter, str::SplitWhitespace};

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    // parse lines
    let reports = input
        .lines()
        .filter(|line| {
            is_safe(
                &line
                    .split_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            )
        })
        .count();
    // check each line if safe and count
    Some(reports as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = input
        .lines()
        .filter(|line| {
            is_safe_with_dampen(
                &line
                    .split_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            )
        })
        .count();
    Some(reports as u32)
}

fn is_safe(input: &Vec<u32>) -> bool {
    // All Increasing or decreasing
    let first = is_increasing(&input) || is_increasing(&input.iter().rev().cloned().collect());

    // Any two adjacent differ by at least one or at most three
    let second = input
        .iter()
        .zip(input.iter().skip(1))
        .all(|(a, b)| a.abs_diff(*b) >= 1 && a.abs_diff(*b) <= 3);
    first && second
}

fn is_increasing(input: &Vec<u32>) -> bool {
    input.iter().zip(input.iter().skip(1)).all(|(a, b)| a <= b)
}

fn is_safe_with_dampen(input: &Vec<u32>) -> bool {
    if is_safe(&input) {
        // Don't bother checking if already safe
        return true;
    }
    // Check if safe when removing one of each
    (0..input.len()).any(|i| {
        let without_item = input
            .iter()
            .enumerate()
            .filter_map(|(j, &num)| {
                if j != i {
                    return Some(num);
                } else {
                    return None;
                }
            })
            .collect();
        is_safe(&without_item)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
