advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .filter_map(|line| {
            Some(
                line.split_whitespace()
                    .filter_map(|x| x.parse::<i32>().ok())
                    .collect::<Vec<i32>>(),
            )
        })
        .collect();

    let num_safe: i32 = reports
        .iter()
        .filter(|report| {
            report.windows(2).all(|w| (1..=3).contains(&(w[1] - w[0])))
                || report.windows(2).all(|w| (1..=3).contains(&(w[0] - w[1])))
        })
        .count() as i32;

    Some(num_safe as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .filter_map(|line| {
            Some(
                line.split_whitespace()
                    .filter_map(|x| x.parse::<i32>().ok())
                    .collect::<Vec<i32>>(),
            )
        })
        .collect();

    let is_valid = |report: &[i32]| {
        report.windows(2).all(|w| (1..=3).contains(&(w[1] - w[0])))
            || report.windows(2).all(|w| (1..=3).contains(&(w[0] - w[1])))
    };

    let num_safe: u32 = reports
        .iter()
        .filter(|report| {
            (0..report.len()).any(|i| {
                is_valid(
                    &report[..i]
                        .iter()
                        .chain(&report[i + 1..])
                        .copied()
                        .collect::<Vec<i32>>(),
                )
            })
        })
        .count() as u32;

    Some(num_safe)
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
