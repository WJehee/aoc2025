use std::fs::read_to_string;
use std::cmp::{min};

pub fn solve(path: &str, part2: bool) -> i64 {
    let mut results = Vec::new();
    for segment in read_to_string(path).unwrap().split(',') {
        let segment = segment.trim();
        let (first_id, last_id) = segment.split_once('-').unwrap();
        println!("{}|{}", first_id, last_id);

        let (f1, f2) = first_id.split_at(first_id.len()/2);
        let (f1, f2): (i64, i64) = (f1.parse().unwrap_or(1), f2.parse().unwrap_or(1));

        let (l1, l2) = last_id.split_at(last_id.len()/2);
        let (l1, l2): (i64, i64) = (l1.parse().unwrap_or(1), l2.parse().unwrap_or(1));

        let first_id: i64 = first_id.parse().unwrap();
        let last_id: i64 = last_id.parse().unwrap();

        let left = min(f1, l1);
        let right = min(f2, l2);
        // println!("{} {}", left, right);
        let mut min = min(left, right);

        while min <= l1 || min <= l2 {
            let total: i64 = format!("{}{}", min, min).parse().unwrap();
            if total >= first_id && total <= last_id {
                // println!("{}", total);
                results.push(total);
            }
            min += 1;
        }
    }
    println!("{:?}", results);
    return results.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_p1() {
        let result = solve("examples/day02.txt", false);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_input_p1() {
        let result = solve("inputs/day02.txt", false);
        assert_eq!(result, 5398419778);
    }
}
