use std::fs::read_to_string;
use std::cmp::max;

pub fn solve(path: &str, part2: bool) -> i32 {
    let mut result = 0;
    for line in read_to_string(path).unwrap().lines() {
        let mut left = -10;
        let mut joltage = -10;
        for (i, c) in line.chars().enumerate() {
            let c = c.to_digit(10).expect("invalid digit") as i32;
            if c > left {
                left = c;
                let mut right = -10;
                for cr in line.chars().skip(i+1) {
                    let cr = cr.to_digit(10).expect("invalid digit") as i32;
                    right = max(right, cr);
                }
                joltage = max(joltage, left * 10 + right);
            }
        }
        result += joltage;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_p1() {
        let result = solve("examples/day03.txt", false);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_input_p1() {
        let result = solve("inputs/day03.txt", false);
        assert_eq!(result, 17100);
    }

    #[test]
    fn test_example_p2() {
        let result = solve("examples/day03.txt", true);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_input_p2() {
        let result = solve("inputs/day03.txt", true);
        assert_eq!(result, 0);
    }
}

