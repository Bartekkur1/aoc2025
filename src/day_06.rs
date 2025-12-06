use std::collections::HashMap;
use std::fs::File;
use std::io::{ BufRead, BufReader };

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum ComputeMode {
    Vertical,
    Horizontal,
}

#[derive(Debug)]
struct Worksheet {
    nums_vertical: Vec<i64>,
    nums_horizontal: Vec<i64>,
    operation: Operation,
}

impl Worksheet {
    fn compute(&self, mode: ComputeMode) -> i64 {
        let nums = match mode {
            ComputeMode::Vertical => &self.nums_vertical,
            ComputeMode::Horizontal => &self.nums_horizontal,
        };

        match self.operation {
            Operation::Add => nums.iter().sum(),
            Operation::Subtract => nums.iter().fold(0, |acc, &x| acc - x),
            Operation::Multiply => nums.iter().product(),
            Operation::Divide => nums.iter().fold(1, |acc, &x| acc / x),
        }
    }
}

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn read_level(file_name: &str) -> Vec<Worksheet> {
        let mut worksheets: Vec<Worksheet> = Vec::new();

        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader
            .lines()
            .map(|l| l.unwrap())
            .collect();

        let num_cols = lines[0].len();
        let mut current_worksheet = Worksheet {
            nums_vertical: Vec::new(),
            nums_horizontal: Vec::new(),
            operation: Operation::Add,
        };
        let mut horizontal_nums_buffer: HashMap<usize, String> = HashMap::new();

        for col_idx in 0..num_cols {
            let mut column_chars: Vec<char> = Vec::new();

            for line in &lines {
                if let Some(ch) = line.chars().nth(col_idx) {
                    column_chars.push(ch);
                }
            }

            let last_char = column_chars.last().cloned().unwrap_or(' ');
            let all_empty = column_chars.iter().all(|&c| c == ' ');
            let column_as_number = column_chars
                .iter()
                .filter(|c| c.is_numeric())
                .collect::<String>();

            for (row_idx, &ch) in column_chars.iter().enumerate() {
                if ch.is_numeric() {
                    let entry = horizontal_nums_buffer.entry(row_idx).or_insert(String::new());
                    entry.push(ch);
                }
            }

            if !column_as_number.is_empty() {
                current_worksheet.nums_vertical.push(column_as_number.parse::<i64>().unwrap());
            }

            if last_char == '+' || last_char == '-' || last_char == '*' || last_char == '/' {
                current_worksheet.operation = match last_char {
                    '+' => Operation::Add,
                    '-' => Operation::Subtract,
                    '*' => Operation::Multiply,
                    '/' => Operation::Divide,
                    _ => Operation::Add,
                };
            }

            if all_empty || col_idx == num_cols - 1 {
                for (_, num_str) in horizontal_nums_buffer.drain() {
                    if !num_str.is_empty() {
                        current_worksheet.nums_horizontal.push(num_str.parse::<i64>().unwrap());
                    }
                }
                // println!("Completed worksheet: {:?}", current_worksheet);
                worksheets.push(current_worksheet);
                current_worksheet = Worksheet {
                    nums_vertical: Vec::new(),
                    nums_horizontal: Vec::new(),
                    operation: Operation::Add,
                };
            }
        }

        worksheets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_level_test() {
        let worksheets = Solution::read_level("./input/day_06_example.txt");
        assert_eq!(worksheets.len(), 4);
    }

    #[test]
    fn solve_part1_example() {
        let worksheets = Solution::read_level("./input/day_06_example.txt");
        let worksheets_results_sum: i64 = worksheets
            .iter()
            .map(|ws| ws.compute(ComputeMode::Horizontal))
            .sum();
        assert_eq!(worksheets_results_sum, 4277556);
    }

    #[test]
    fn solve_part1() {
        let worksheets = Solution::read_level("./input/day_06.txt");
        let worksheets_results_sum: i64 = worksheets
            .iter()
            .map(|ws| ws.compute(ComputeMode::Horizontal))
            .sum();
        assert_eq!(worksheets_results_sum, 4951502530386);
    }

    #[test]
    fn solve_part2_example() {
        let worksheets = Solution::read_level("./input/day_06_example.txt");
        let worksheets_results_sum: i64 = worksheets
            .iter()
            .map(|ws| ws.compute(ComputeMode::Vertical))
            .sum();
        assert_eq!(worksheets_results_sum, 3263827);
    }

    #[test]
    fn solve_part2() {
        let worksheets = Solution::read_level("./input/day_06.txt");
        let worksheets_results_sum: i64 = worksheets
            .iter()
            .map(|ws| ws.compute(ComputeMode::Vertical))
            .sum();
        assert_eq!(worksheets_results_sum, 8486156119946);
    }
}
