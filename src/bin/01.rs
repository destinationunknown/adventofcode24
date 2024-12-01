advent_of_code::solution!(1);

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
    None
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
        assert_eq!(result, None);
    }
}
