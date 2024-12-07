advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    // Iterate over each char
    // Check if the next 3 aree ul(#
    // Next char must be #, || ##, || ,
    input
        .chars()
        .enumerate()
        .map(|(i, ch)| validate(input.chars().skip(i)));
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn validate<T>(input: T) -> bool
where
    T: Iterator<Item = char>,
{
    // Check if next n chars are mul(
    let keyword = "mul(".chars();
    keyword.zip(input).all(|(k, inp)| k == inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_validate() {
        let correct = validate("mul(89,59)".chars());
        assert!(correct);

        let incorrect = validate("90mul".chars());
        assert!(!incorrect);

        let tricky = validate("1mul(89,59".chars());
        assert!(!tricky);
    }
}
