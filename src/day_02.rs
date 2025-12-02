use std::fs::File;
use std::io::{ BufRead, BufReader };

struct Solution;

#[derive(PartialEq, Copy, Clone)]
enum Pattern {
    Silly,
    Regular,
}

struct Range {
    min: u64,
    max: u64,
}

impl Range {
    pub fn from_string(s: &str) -> Vec<Range> {
        let parts: Vec<&str> = s.split(',').collect();
        parts
            .iter()
            .map(|part| {
                let bounds: Vec<&str> = part.split('-').collect();
                Range {
                    min: bounds[0].parse::<u64>().unwrap(),
                    max: bounds[1].parse::<u64>().unwrap(),
                }
            })
            .collect()
    }
}

struct RangeValidator {
    pattern: Pattern,
}

impl RangeValidator {
    pub fn count_valid(&self, range: &Range) -> i64 {
        let mut valid_count: i64 = 0;
        for val in range.min..=range.max {
            if self.pattern == Pattern::Regular {
                if !RangeValidator::is_valid_value(&val.to_string()) {
                    valid_count += val as i64;
                }
            } else {
                if !RangeValidator::is_valid_value_silly(&val.to_string()) {
                    valid_count += val as i64;
                }
            }
        }
        valid_count
    }

    pub fn is_valid_value(val: &str) -> bool {
        if val.len() % 2 != 0 {
            return true;
        }

        let half_val = val.len() / 2;
        let (first_half, second_half) = val.split_at(half_val);
        return first_half != second_half;
    }

    pub fn is_valid_value_silly(val: &str) -> bool {
        let mut pattern = Vec::<char>::new();

        for char in val.chars() {
            pattern.push(char);

            let pattern_str: String = pattern.iter().collect();
            let value_split = val.split(pattern_str.as_str());
            let all_empty = value_split.clone().all(|part| part.is_empty());

            if all_empty && pattern.len() != val.len() {
                return false;
            }
        }

        true
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn solve(file_path: &str, pattern: Pattern) -> i64 {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        let first_line = reader.lines().next().unwrap().unwrap();

        let mut valid_count: i64 = 0;
        let range = Range::from_string(&first_line);
        let validator = RangeValidator { pattern };
        for r in range.iter() {
            valid_count += validator.count_valid(r);
        }
        println!(
            "Day02 {} solution: {}",
            match pattern {
                Pattern::Regular => "Part 1",
                Pattern::Silly => "Part 2",
            },
            valid_count
        );
        valid_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_part_2_example() {
        Solution::solve("./input/day_02_example.txt", Pattern::Silly);
    }

    #[test]
    fn solve_part_2() {
        Solution::solve("./input/day_02.txt", Pattern::Silly);
    }

    #[test]
    fn solve_part_1_example() {
        Solution::solve("./input/day_02_example.txt", Pattern::Regular);
    }

    #[test]
    fn solve_part_1() {
        Solution::solve("./input/day_02.txt", Pattern::Regular);
    }

    #[test]
    fn test_values() {
        let incorrect = vec![
            "11",
            "22",
            "99",
            "1010",
            "1188511885",
            "222222",
            "446446",
            "38593859"
        ];
        for val in incorrect.iter() {
            assert_eq!(RangeValidator::is_valid_value(val), false);
        }

        let correct = vec!["12", "101"];
        for val in correct.iter() {
            assert_eq!(RangeValidator::is_valid_value(val), true);
        }
    }

    #[test]
    fn test_silly_pattern() {
        assert_eq!(RangeValidator::is_valid_value_silly("12341234"), false);
        assert_eq!(RangeValidator::is_valid_value_silly("123123123"), false);
        assert_eq!(RangeValidator::is_valid_value_silly("1212121212"), false);
        assert_eq!(RangeValidator::is_valid_value_silly("1111111"), false);
    }

    #[test]
    fn test_silly_range() {
        let validator = RangeValidator { pattern: Pattern::Silly };
        assert_eq!(validator.count_valid(&(Range { min: 95, max: 115 })), 210);
        assert_eq!(validator.count_valid(&(Range { min: 998, max: 1012 })), 2009);
        assert_eq!(
            validator.count_valid(&(Range { min: 1188511880, max: 1188511890 })),
            1188511885
        );
    }

    #[test]
    fn test_range() {
        let validator = RangeValidator { pattern: Pattern::Regular };
        assert_eq!(validator.count_valid(&(Range { min: 11, max: 22 })), 33);
        assert_eq!(validator.count_valid(&(Range { min: 95, max: 115 })), 99);
        assert_eq!(
            validator.count_valid(&(Range { min: 1188511880, max: 1188511890 })),
            1188511885
        );
    }
}
