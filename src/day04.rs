use std::fs::read_to_string;

pub fn solve(path: &str, part2: bool) -> i64 {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in read_to_string(path).unwrap().lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    let mut result = 0;
    loop {
        let (ans, new_grid) = remove(grid.clone());
        result += ans;
        if !part2 || ans == 0 { break; }
        grid = new_grid;
    }
    result
}

fn remove(grid: Vec<Vec<char>>) -> (i64, Vec<Vec<char>>) {
    let mut new_grid = Vec::new();
    let mut result = 0;
    for (i, _row) in grid.iter().enumerate() {
        let mut new_row = Vec::new();
        for (j, item) in _row.iter().enumerate() {
            let mut roll_count = -1;
            if *item == '@' {
                roll_count = 0;
                let mut ids = vec![i, i+1];
                if i != 0 {
                    ids.insert(0, i-1);
                }
                let rows: Vec<_> = ids.iter()
                    .filter_map(|&x| grid.get(x))
                    .collect();
                for row in &rows {
                    let mut ids = vec![j, j+1];
                    if j != 0 {
                        ids.insert(0, j-1);
                    }
                    let items: Vec<_> = ids.iter()
                        .filter_map(|&x| row.get(x))
                        .collect();

                    for char in items {
                        if *char == '@' { roll_count += 1; }
                    }
                }
            }
            if roll_count > 0 && roll_count <= 4 { 
                result += 1;
                print!("x");
                new_row.push('x');
            }  else {
                print!("{}", item);
                new_row.push(*item);
            }
        }
        println!();
        new_grid.push(new_row);
    }
    println!();
    (result, new_grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_p1() {
        let result = solve("examples/day04.txt", false);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_input_p1() {
        let result = solve("inputs/day04.txt", false);
        assert_eq!(result, 1349);
    }

    #[test]
    fn test_example_p2() {
        let result = solve("examples/day04.txt", true);
        assert_eq!(result, 43);
    }

    #[test]
    fn test_input_p2() {
        let result = solve("inputs/day04.txt", true);
        assert_eq!(result, 8277);
    }
}

