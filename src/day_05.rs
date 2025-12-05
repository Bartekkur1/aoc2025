use std::fs::File;
use std::io::{ BufRead, BufReader };

struct FreshRange {
    start: i64,
    end: i64,
}

impl FreshRange {
    pub fn from_string(str: &str) -> Self {
        let parts: Vec<&str> = str.split('-').collect();
        let start = parts[0].parse::<i64>().unwrap();
        let end = parts[1].parse::<i64>().unwrap();

        FreshRange { start, end }
    }

    pub fn contains(&self, val: i64) -> bool {
        self.start <= val && val <= self.end
    }

    pub fn elements_count_inclusive(&self) -> i64 {
        self.end - self.start + 1
    }
}

struct Input {
    pub ranges: Vec<FreshRange>,
    pub values: Vec<i64>,
}

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge_ranges(input: &Input) -> Vec<FreshRange> {
        let mut ranges: Vec<FreshRange> = input.ranges
            .iter()
            .map(|r| FreshRange { start: r.start, end: r.end })
            .collect();

        ranges.sort_by_key(|r| r.start);

        let mut merged: Vec<FreshRange> = Vec::new();

        for range in ranges {
            if merged.is_empty() {
                merged.push(range);
            } else {
                let last_idx = merged.len() - 1;
                let last = &merged[last_idx];

                if range.start <= last.end + 1 {
                    merged[last_idx].end = merged[last_idx].end.max(range.end);
                } else {
                    merged.push(range);
                }
            }
        }

        merged
    }

    pub fn solve_part2(input: &Input) -> i64 {
        let mut count = 0;

        for range in &input.ranges {
            count += range.elements_count_inclusive();
        }

        count
    }

    pub fn solve_part1(input: &Input) -> i64 {
        let mut count = 0;

        for &value in &input.values {
            for range in &input.ranges {
                if range.contains(value) {
                    count += 1;
                    break;
                }
            }
        }

        count
    }

    pub fn read_input(file_name: &str) -> Input {
        let mut reading_ranges = true;
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);
        let mut input = Input {
            ranges: Vec::new(),
            values: Vec::new(),
        };

        for line in reader.lines() {
            let line = line.unwrap();

            if line == "" {
                reading_ranges = false;
                continue;
            }

            if reading_ranges {
                let range = FreshRange::from_string(&line);
                input.ranges.push(range);
            } else {
                let value = line.parse::<i64>().unwrap();
                input.values.push(value);
            }
        }

        input
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_range() {
        let input = "2-4";
        let range = FreshRange::from_string(input);

        assert_eq!(range.start, 2);
        assert_eq!(range.end, 4);
    }

    #[test]
    fn test_range_contains() {
        let range = FreshRange { start: 2, end: 4 };

        assert!(range.contains(2));
        assert!(range.contains(3));
        assert!(range.contains(4));
        assert!(!range.contains(1));
        assert!(!range.contains(5));
    }

    #[test]
    fn test_input_read() {
        let input = Solution::read_input("./input/day_05_example.txt");

        assert_eq!(input.ranges.len(), 4);
        assert_eq!(input.values.len(), 6);
    }

    #[test]
    fn test_part1_example() {
        let input = Solution::read_input("./input/day_05_example.txt");
        let result = Solution::solve_part1(&input);

        assert_eq!(result, 3);
    }

    #[test]
    fn test_part1() {
        let input = Solution::read_input("./input/day_05.txt");
        let result = Solution::solve_part1(&input);

        assert_eq!(result, 617);
    }

    #[test]
    fn merge_ranges() {
        let input = Input {
            ranges: vec![
                FreshRange { start: 3, end: 5 },
                FreshRange { start: 10, end: 14 },
                FreshRange { start: 16, end: 20 },
                FreshRange { start: 12, end: 18 }
            ],
            values: vec![],
        };

        let merged = Solution::merge_ranges(&input);

        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].start, 3);
        assert_eq!(merged[0].end, 5);
        assert_eq!(merged[1].start, 10);
        assert_eq!(merged[1].end, 20);
    }

    #[test]
    fn test_elements_count_inclusive() {
        let range1 = FreshRange { start: 3, end: 5 };
        assert_eq!(range1.elements_count_inclusive(), 3);

        let range2 = FreshRange { start: 10, end: 20 };
        assert_eq!(range2.elements_count_inclusive(), 11);
    }

    #[test]
    fn solve_part2_example() {
        let input = Solution::read_input("./input/day_05_example.txt");
        let merged_ranges = Solution::merge_ranges(&input);
        let merged_input = Input {
            ranges: merged_ranges,
            values: vec![],
        };
        let result = Solution::solve_part2(&merged_input);

        assert_eq!(result, 14);
    }

    #[test]
    fn solve_part2() {
        let input = Solution::read_input("./input/day_05.txt");
        let merged_ranges = Solution::merge_ranges(&input);
        let merged_input = Input {
            ranges: merged_ranges,
            values: vec![],
        };
        let result = Solution::solve_part2(&merged_input);

        assert_eq!(result, 14);
    }
}
