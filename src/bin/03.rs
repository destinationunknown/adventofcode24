advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let result = re
        .captures_iter(input)
        .filter_map(|x| {
            let first = x[1].parse::<u32>().ok()?;
            let second = x[2].parse::<u32>().ok()?;
            Some(first * second)
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let without_newlines = input.replace('\n', "");
    let dont_do_re = Regex::new(r"don't\(\).*?(do\(\)|$)").unwrap();
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let filtered = dont_do_re.replace_all(&without_newlines, "");
    let result: u32 = mul_re
        .captures_iter(&filtered)
        .filter_map(|x| {
            let first = x[1].parse::<u32>().ok()?;
            let second = x[2].parse::<u32>().ok()?;
            Some(first * second)
        })
        .sum();

    Some(result)
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
