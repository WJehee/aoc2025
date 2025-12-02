use std::fs::read_to_string;

pub fn solve(path: &str, part2: bool) -> i32 {
    let mut dial = 50;
    let mut zero_count = 0;
    for line in read_to_string(path).unwrap().lines() {

        let (direction, rotation) = line.split_at(1);
        let rotation: i32 = rotation.parse().expect("not a valid number");
        let modifier = match direction {
            "L" => -1,
            "R" => 1,
            _ => panic!("Invalid first character"),
        };
        let was_zero = dial == 0;
        dial += rotation * modifier; 
     
        let mut passes = 0;
        while dial < 0 || dial > 99 {
            if dial < 0 {
                dial += 100;
            } else if dial > 99 {
                dial -= 100;
            }
            if part2 { passes += 1; }
        }
        if part2 && modifier < 0 {
            if dial == 0 { passes += 1; }
            if was_zero { passes -= 1; }
        }
        if !part2 && dial == 0 { passes += 1; }

        zero_count += passes;
    }
    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_p1() {
        let result = solve("examples/day1.txt", false);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_input_p1() {
        let result = solve("inputs/day1.txt", false);
        assert_eq!(result, 1064);
    }

    #[test]
    fn test_example_p2() {
        let result = solve("examples/day1.txt", true);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_input_p2() {
        let result = solve("inputs/day1.txt", true);
        assert_eq!(result, 6122);
    }
}
