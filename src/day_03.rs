use std::fs::File;
use std::io::{ BufRead, BufReader };

struct Battery {
    bank: String,
    capacity: i64,
}

impl Battery {
    pub fn from_string(s: &str, capacity_len: i32) -> Self {
        let mut capacity_vec = Vec::<String>::new();
        let mut last_index: i32 = 0;

        for i in 0..capacity_len {
            let [num, last_index_offset] = Battery::find_max(
                s.split_at(last_index as usize).1,
                capacity_len - i - 1
            );
            capacity_vec.push(num.to_string());
            last_index += last_index_offset + 1;
        }

        Battery {
            bank: s.to_string(),
            capacity: capacity_vec.join("").parse::<i64>().unwrap(),
        }
    }

    fn find_max(s: &str, size: i32) -> [i32; 2] {
        let nums_vec = s
            .chars()
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut largest_num = 0;
        let mut largest_index = 0;
        for (index, value) in nums_vec.iter().enumerate() {
            if index == nums_vec.len() - (size as usize) {
                break;
            }

            if *value > largest_num && index < nums_vec.len() {
                largest_num = *value;
                largest_index = index;
            }

            if nums_vec.len() == 1 {
                largest_num = *value;
                largest_index = index;
            }
        }

        [largest_num, largest_index as i32]
    }
}

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn build_input(file_name: &str, capacity_len: i32) -> Vec<Battery> {
        let mut battery_vec: Vec<Battery> = Vec::new();
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line;
            let line_value = line.unwrap();
            let battery = Battery::from_string(&line_value, capacity_len);
            battery_vec.push(battery);
        }

        battery_vec
    }

    pub fn solve(batteries: Vec<Battery>) -> i64 {
        batteries
            .iter()
            .map(|b| b.capacity)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn battery_test_1() {
        assert_eq!(Battery::from_string("987654321111111", 2).capacity, 98);
        assert_eq!(Battery::from_string("811111111111119", 2).capacity, 89);
        assert_eq!(Battery::from_string("811111111111119", 3).capacity, 819);
        assert_eq!(Battery::from_string("234234234234278", 2).capacity, 78);
        assert_eq!(Battery::from_string("818181911112111", 2).capacity, 92);
    }

    #[test]
    fn solution_part1_example() {
        let batteries = Solution::build_input("./input/day_03_example.txt", 2);
        assert_eq!(batteries.len(), 4);

        let sum = Solution::solve(batteries);
        assert_eq!(sum, 357);
    }

    #[test]
    fn solution_part1() {
        let batteries = Solution::build_input("./input/day_03.txt", 2);
        let sum = Solution::solve(batteries);
        assert_eq!(sum, 16946);
    }

    #[test]
    fn battery_test_2() {
        assert_eq!(Battery::from_string("987654321111111", 12).capacity, 987654321111);
        assert_eq!(Battery::from_string("811111111111119", 12).capacity, 811111111119);
        assert_eq!(Battery::from_string("234234234234278", 12).capacity, 434234234278);
        assert_eq!(Battery::from_string("818181911112111", 12).capacity, 888911112111);
    }

    #[test]
    fn solution_part2_example() {
        let batteries = Solution::build_input("./input/day_03_example.txt", 12);
        assert_eq!(batteries.len(), 4);

        let sum = Solution::solve(batteries);
        println!("Part 2 example solution: {}", sum);
    }

    #[test]
    fn solution_part2() {
        let batteries = Solution::build_input("./input/day_03.txt", 12);
        let sum = Solution::solve(batteries);
        println!("Part 2 solution: {}", sum);
    }
}
