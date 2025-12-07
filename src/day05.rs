use std::fs::read_to_string;

enum Stages {
    Ranges,
    Ids,
}

pub fn solve(path: &str, part2: bool) -> i64 {
    let mut stage = Stages::Ranges;
    let mut ranges = Vec::new();
    let mut result = 0;
    for line in read_to_string(path).unwrap().lines() {
        match stage {
            Stages::Ranges => {
                if line == "" {
                    stage = Stages::Ids;
                } else {
                    let (lower, upper) = line.split_once("-").unwrap();
                    let (lower, upper): (i64, i64) = (lower.parse().unwrap(), upper.parse().unwrap());
                    // println!("{}-{}", lower, upper);
                    ranges.push((lower, upper));
                }
            },
            Stages::Ids => {
                let id: i64 = line.parse().unwrap();
                for (lower, upper) in ranges.iter() {
                    println!("{}>{}<{}", lower, id, upper);
                    if id >= *lower && id <= *upper {
                        result += 1;
                        break;
                    }
                }
            },
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_p1() {
        let result = solve("examples/day05.txt", false);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_input_p1() {
        let result = solve("inputs/day05.txt", false);
        assert_eq!(result, 885);
    }

    // #[test]
    // fn test_example_p2() {
    //     let result = solve("examples/day05.txt", true);
    //     assert_eq!(result, -1);
    // }
    //
    // #[test]
    // fn test_input_p2() {
    //     let result = solve("inputs/day05.txt", true);
    //     assert_eq!(result, -1);
    // }
}

