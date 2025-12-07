use std::fs::read_to_string;
use std::cmp::max;

pub fn solve(path: &str, part2: bool) -> i64 {
    let mut batteries = 2;
    if part2 { batteries = 12; }

    let mut result = 0;
    for line in read_to_string(path).unwrap().lines() {
        let joltage = joltage(line, batteries);
        result += joltage;
    }
    result
}

fn joltage(line: &str, batteries: usize) -> i64 {
    if line.len() < batteries { return 0; }
    if batteries == 1 {
        let mut max_battery = 0;
        for c in line.chars() {
            let c = c.to_digit(10).expect("invalid digit") as i64;
            max_battery = max(max_battery, c);
        }
        return max_battery;
    }
    for num in (0..10).rev() {
        for (i, c) in line.chars().enumerate() {
            let c = c.to_digit(10).expect("invalid digit") as i64;
            if c != num { continue; }

            let jolt = joltage(&line[i+1..], batteries-1);
            if jolt > 0 {
                let jolt = format!("{}{}", c, jolt).parse().unwrap();
                return jolt;
            }
        }
    }
    0
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
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn test_input_p2() {
        let result = solve("inputs/day03.txt", true);
        assert_eq!(result, 170418192256861);
    }
}

