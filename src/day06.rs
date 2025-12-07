use std::fs::read_to_string;

pub fn solve(path: &str, part2: bool) -> i64 {
    let mut columns = Vec::new();
    let mut total = 0;
    for line in read_to_string(path).unwrap().lines() {
        let row = line.split(" ").map(|i| i.trim()).filter(|i| *i != "");
        for (i, item) in row.enumerate() {
            if columns.get(i).is_none() { columns.push(Vec::new()); }
            if let Some(col) = columns.get_mut(i) {
                match item {
                    "+" => {
                        let sum: i64 = col.iter().sum();
                        total += sum;
                    },
                    "*" => {
                        total += col.iter().fold(1, |total, new| total * new);
                    },
                    num => {
                        let num: i64 = num.parse().unwrap();
                        col.push(num);
                    },
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_p1() {
        let result = solve("examples/day06.txt", false);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_input_p1() {
        let result = solve("inputs/day06.txt", false);
        assert_eq!(result, 6209956042374);
    }
    
    // #[test]
    // fn test_example_p2() {
    //     let result = solve("examples/day06.txt", true);
    //     assert_eq!(result, -1);
    // }
    //
    // #[test]
    // fn test_input_p2() {
    //     let result = solve("inputs/day06.txt", true);
    //     assert_eq!(result, -1);
    // }
}

