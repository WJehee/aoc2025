use std::fs::read_to_string;

enum Stages {
    Ranges,
    Ids,
}

struct IdRanges {
    pub ranges: Vec<(i64, i64)>,
}

impl IdRanges {
    fn new() -> Self {
        IdRanges { ranges: Vec::new() }
    }

    fn show(&self) {
        for (l, u) in self.ranges.iter() {
            println!("{}-{}", l, u);
        }
    }

    fn within(&self, n: i64) -> bool {
        for (l, u) in self.ranges.iter() {
            if n >= *l && n <= *u { return true; }
        }
        false
    }

    fn add_range(&mut self, lower: i64, upper: i64) {
        self.ranges.push((lower, upper))
    }

    fn reduce(&mut self) {
        'main: loop {
            for i in 0..self.ranges.len() {
                for j in (i+1)..self.ranges.len() {
                    let (l1, u1) = self.ranges[i];
                    let (l2, u2) = self.ranges[j];

                    if l1 >= l2 && u1 <= u2 {
                        self.ranges.remove(i);
                        continue 'main;
                    }
                    if l2 >= l1 && u2 <= u1 {
                        self.ranges.remove(j);
                        continue 'main;
                    }
                    if l1 <= u2 && l2 <= u1 {
                        let new_range = (l1.min(l2), u1.max(u2));
                        self.ranges[i] = new_range;
                        self.ranges.remove(j);
                        continue 'main;
                    }
                }
            }
            break;
        }
    }
}

pub fn solve(path: &str, part2: bool) -> i64 {
    let mut stage = Stages::Ranges;
    let mut ranges = IdRanges::new();
    let mut result = 0;
    for line in read_to_string(path).unwrap().lines() {
        match stage {
            Stages::Ranges => {
                if line == "" {
                    stage = Stages::Ids;
                } else {
                    let (lower, upper) = line.split_once("-").unwrap();
                    let (lower, upper): (i64, i64) = (lower.parse().unwrap(), upper.parse().unwrap());
                    ranges.add_range(lower, upper);
                }
            },
            Stages::Ids => {
                if !part2 {
                    let id: i64 = line.parse().unwrap();
                    if ranges.within(id) {
                        result += 1;
                    }
                } else {
                    ranges.reduce();

                    for (lower, upper) in ranges.ranges.iter() {
                        result += upper - lower + 1; 
                    }
                    break;
                }
            },
        }
    }
    ranges.show();
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

    #[test]
    fn test_example_p2() {
        let result = solve("examples/day05.txt", true);
        assert_eq!(result, 14);
    }
    
    #[test]
    fn test_input_p2() {
        let result = solve("inputs/day05.txt", true);
        assert_eq!(result, 348115621205535);
    }
}

