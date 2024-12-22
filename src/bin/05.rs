advent_of_code::solution!(5);

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let parts = input.splitn(2, "\n\n").collect::<Vec<&str>>();
    let rules_section = parts[0];
    let updates_section = parts[1];
    let rules: Vec<(i32, i32)> = rules_section
        .lines()
        .map(|line| {
            let l: Vec<&str> = line.splitn(2, '|').collect();
            (l[0].parse().unwrap(), l[1].parse().unwrap())
        })
        .collect();

    let updates: Vec<Vec<i32>> = updates_section
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect();

    updates
        .iter()
        .filter(|update| {
            // combinations will keep pairs in their relative ordering from the original vector
            update.iter().combinations(2).all(|pair| {
                let first = pair[0];
                let second = pair[1];
                let invalid = rules.iter().any(|(x, y)| first == y && second == x);

                !invalid
            })
        })
        .map(|update| update[update.len() / 2])
        .sum::<i32>()
        .try_into()
        .ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
