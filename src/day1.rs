use std::fs::read_to_string;

pub fn solve(path: &str) -> i32 {
    let mut start = 50;
    let mut zero_count = 0;
    for line in read_to_string(path).unwrap().lines() {
        if start == 0 { zero_count += 1; }

        let (direction, rotation) = line.split_at(1);
        let rotation: i32 = rotation.parse().expect("not a valid number");
        match direction {
            "L" => start = rotate_left(start, rotation),
            "R" => start = rotate_right(start, rotation),
            _ => panic!("Invalid first character"),
        }
    }
    zero_count
}

fn rotate_left(current: i32, rotation: i32) -> i32 {
    let mut total = current - rotation;
    while total < 0 {
        total += 100;
    }
    total
}

fn rotate_right(current: i32, rotation: i32) -> i32 {
    let mut total = current + rotation;
    while total >= 100 {
        total -= 100;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_p1() {
        let mut start = 50;

        start = rotate_left(start, 68);
        assert_eq!(start, 82);

        start = rotate_left(start, 30);
        assert_eq!(start, 52);

        start = rotate_right(start, 48);
        assert_eq!(start, 0);
    }

    #[test]
    fn test_example_p1() {
        let result = solve("examples/day1.txt");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_input_p1() {
        let result = solve("inputs/day1.txt");
        assert_eq!(result, 1064);
    }
}
