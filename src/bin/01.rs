advent_of_code::solution!(1);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    // read input into two lists
    let (mut a, mut b): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let first = parts.next()?.parse::<u32>().ok()?;
            let second = parts.next()?.parse::<u32>().ok()?;
            Some((first, second))
        })
        .unzip();

    a.sort();
    b.sort();

    let sum_diff: u32 = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| if x > y { x - y } else { y - x })
        .sum();

    Some(sum_diff)
}

pub fn part_two(input: &str) -> Option<u32> {
    // read input into two lists
    let (a, b): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let first = parts.next()?.parse::<u32>().ok()?;
            let second = parts.next()?.parse::<u32>().ok()?;
            Some((first, second))
        })
        .unzip();

    let b_freq = b.into_iter().fold(HashMap::new(), |mut map, val| {
        *map.entry(val).or_insert(0) += 1;
        map
    });

    let similarity_score: u32 = a
        .iter()
        .map(|x| x * b_freq.get(x).unwrap_or(&0))
        .sum();

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
