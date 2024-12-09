advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let m = matrix.len() as i32;
    let n = matrix[0].len() as i32;

    let check_pos = |row: i32, col: i32| -> i32 {
        let xmas = "XMAS";
        let dirs = [
            (0, 1),
            (0, -1),
            (-1, 0),
            (1, 0),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];

        let mut result = 0;

        'outer: for (dr, dc) in dirs {
            for i in 0..4 {
                let (r, c) = (row + (i * dr), col + (i * dc));
                if r >= 0 && c >= 0 && r < m && c < n {
                    if xmas.chars().nth(i as usize).unwrap()
                        != matrix[r as usize][c as usize]
                    {
                        continue 'outer;
                    }
                } else {
                    continue 'outer;
                }
            }
            result += 1;
        }

        return result;
    };

    let mut count = 0;

    for i in 0..m {
        for j in 0..n {
            count += check_pos(i, j);
        }
    }

    Some(count as u32)
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
