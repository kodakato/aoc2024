advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    // Parse into two arrays
    for line in input.lines() {
        let mut nums = line
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap());
        left.push(nums.next().unwrap());
        right.push(nums.next().unwrap());
    }

    // Sort arrays
    left.sort_unstable();
    right.sort_unstable();

    let total_distance: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    // Parse into two arrays
    for line in input.lines() {
        let mut nums = line
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap());
        left.push(nums.next().unwrap());
        right.push(nums.next().unwrap());
    }

    let sim_score: usize = left
        .iter()
        .map(|left_num| {
            right
                .iter()
                .filter(|&right_num| right_num == left_num)
                .count()
                * (*left_num as usize)
        })
        .sum();
    Some(sim_score as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }
}
