use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::collections::HashSet;

struct Solution;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Solution {
    fn count_adjacent_occupied(pos: &Position, occupied: &HashSet<Position>) -> usize {
        const DELTAS: [(i32, i32); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        DELTAS.iter()
            .filter(|(dx, dy)| {
                occupied.contains(
                    &(Position {
                        x: pos.x + dx,
                        y: pos.y + dy,
                    })
                )
            })
            .count()
    }

    fn find_accessible(occupied: &HashSet<Position>) -> Vec<Position> {
        occupied
            .iter()
            .filter(|pos| Solution::count_adjacent_occupied(pos, occupied) < 4)
            .copied()
            .collect()
    }

    fn read_input(file_name: &str) -> HashSet<Position> {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);
        let mut occupied = HashSet::new();

        for (y, line) in reader.lines().enumerate() {
            for (x, c) in line.unwrap().chars().enumerate() {
                if c == '@' {
                    occupied.insert(Position {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }

        occupied
    }

    pub fn solve_part1(file_name: &str) -> usize {
        let occupied = Solution::read_input(file_name);
        Solution::find_accessible(&occupied).len()
    }

    pub fn solve_part2(file_name: &str) -> i32 {
        let mut occupied = Solution::read_input(file_name);
        let mut total_removed = 0;

        loop {
            let accessible = Solution::find_accessible(&occupied);
            if accessible.is_empty() {
                break;
            }

            total_removed += accessible.len() as i32;
            for pos in accessible {
                occupied.remove(&pos);
            }
        }

        total_removed
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_part_1() {
        let accessible = Solution::solve_part1("./input/day_04_example.txt");
        assert_eq!(accessible, 13);
    }

    #[test]
    fn test_case_part_2() {
        let removed = Solution::solve_part2("./input/day_04_example.txt");
        assert_eq!(removed, 43);
    }

    #[test]
    fn solve_part1() {
        let accessible = Solution::solve_part1("./input/day_04.txt");
        assert_eq!(accessible, 1428);
    }

    #[test]
    fn solve_part2() {
        let removed = Solution::solve_part2("./input/day_04.txt");
        assert_eq!(removed, 8936);
    }
}
