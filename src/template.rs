use std::fs::read_to_string;

pub fn solve(path: &str, part2: bool) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_p1() {
        let result = solve("examples/dayxx.txt", false);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_input_p1() {
        let result = solve("inputs/dayxx.txt", false);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_example_p2() {
        let result = solve("examples/dayxx.txt", true);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_input_p2() {
        let result = solve("inputs/dayxx.txt", true);
        assert_eq!(result, -1);
    }
}

