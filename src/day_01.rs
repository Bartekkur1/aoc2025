use std::fs::File;
use std::io::{ BufRead, BufReader };

struct DialRange {
    pub min: i32,
    pub max: i32,
}

struct DialConfig {
    pub range: DialRange,
    pub start_position: i32,
}

struct DialClock {
    pub position: i32,
    pub config: DialConfig,
}

#[derive(Debug, PartialEq)]
enum DialDirection {
    Left,
    Right,
}

#[derive(PartialEq)]
enum DialDirectionBuildType {
    Single,
    Explode,
}

#[derive(Debug)]
struct DialMovement {
    pub steps: i32,
    pub direction: DialDirection,
}

impl DialMovement {
    pub fn from_string(s: &str) -> Self {
        let direction = s.chars().next().unwrap();
        let steps: i32 = s[1..].parse().unwrap();

        let dial_direction = match direction {
            'L' => DialDirection::Left,
            'R' => DialDirection::Right,
            _ => panic!("Invalid direction"),
        };

        DialMovement {
            steps,
            direction: dial_direction,
        }
    }

    pub fn from_string_explode(s: &str) -> Vec<Self> {
        let (dir_char, steps_str) = s.split_at(1);
        let steps: i32 = steps_str.parse().unwrap();

        let dial_direction = match dir_char {
            "L" => DialDirection::Left,
            "R" => DialDirection::Right,
            _ => panic!("Invalid direction"),
        };

        (0..steps)
            .map(|_| DialMovement {
                steps: 1,
                direction: if dial_direction == DialDirection::Left {
                    DialDirection::Left
                } else {
                    DialDirection::Right
                },
            })
            .collect()
    }
}

impl DialClock {
    pub fn new(config: DialConfig) -> Self {
        DialClock {
            position: config.start_position,
            config,
        }
    }

    fn turn(&mut self, movement: DialMovement) {
        let delta = match movement.direction {
            DialDirection::Left => -movement.steps,
            DialDirection::Right => movement.steps,
        };
        self.position += delta;
        let range_size = self.config.range.max - self.config.range.min + 1;
        self.position =
            self.config.range.min + (self.position - self.config.range.min).rem_euclid(range_size);
    }

    pub fn get_position(&self) -> i32 {
        self.position
    }
}

struct Solution {}

impl Solution {
    fn iterate_input<F>(self, file_name: &str, build_type: DialDirectionBuildType, mut callback: F)
        where F: FnMut(DialMovement)
    {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line;
            if build_type == DialDirectionBuildType::Explode {
                let movement = DialMovement::from_string_explode(&line.unwrap());
                for m in movement {
                    callback(m);
                }
            } else {
                let movement = vec![DialMovement::from_string(&line.unwrap())];
                for m in movement {
                    callback(m);
                }
            }
        }
    }

    fn run(self, build_type: DialDirectionBuildType, file_name: &str) -> i32 {
        let mut zero_count = 0;
        let mut dial = DialClock::new(DialConfig {
            range: DialRange { min: 0, max: 99 },
            start_position: 50,
        });

        self.iterate_input(file_name, build_type, |movement| {
            dial.turn(movement);
            if dial.get_position() == 0 {
                zero_count += 1;
            }
        });

        return zero_count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> DialClock {
        DialClock::new(DialConfig {
            range: DialRange { min: 0, max: 99 },
            start_position: 50,
        })
    }

    #[test]
    fn test_solution_run_on_example_part1() {
        let solution = Solution {};
        let res = solution.run(DialDirectionBuildType::Single, "./input/day_01_example.txt");
        println!("Example result: {}", res);
    }

    #[test]
    fn test_solution_run_on_input_part1() {
        let solution = Solution {};
        let res = solution.run(DialDirectionBuildType::Single, "./input/day_01.txt");
        println!("Result: {}", res);
    }

    #[test]
    fn test_solution_run_on_example_part2() {
        let solution = Solution {};
        let res = solution.run(DialDirectionBuildType::Explode, "./input/day_01_example.txt");
        println!("Example result part 2: {}", res);
    }

    #[test]
    fn test_solution_run_on_input_part2() {
        let solution = Solution {};
        let res = solution.run(DialDirectionBuildType::Explode, "./input/day_01.txt");
        println!("Result part 2: {}", res);
    }

    #[test]
    fn test_movement_build_explode() {
        let test1 = DialMovement::from_string_explode("R10");
        assert_eq!(test1.len(), 10);
        for movement in test1 {
            assert_eq!(movement.steps, 1);
            assert_eq!(movement.direction, DialDirection::Right);
        }
    }

    #[test]
    fn test_movement_build() {
        let test1 = DialMovement::from_string("R10");
        assert_eq!(test1.steps, 10);
        assert_eq!(test1.direction, DialDirection::Right);

        let test2 = DialMovement::from_string("L25");
        assert_eq!(test2.steps, 25);
        assert_eq!(test2.direction, DialDirection::Left);
    }

    #[test]
    fn test_dial_init() {
        let dial = setup();

        assert_eq!(dial.get_position(), 50);
    }

    #[test]
    fn test_dial_movement_1() {
        let mut dial = setup();

        let move_right = DialMovement {
            steps: 10,
            direction: DialDirection::Right,
        };

        dial.turn(move_right);

        assert_eq!(dial.get_position(), 60);
    }

    #[test]
    fn test_dial_movement_2() {
        let mut dial = setup();

        let move_right = DialMovement {
            steps: 20,
            direction: DialDirection::Right,
        };

        let move_left = DialMovement {
            steps: 15,
            direction: DialDirection::Left,
        };

        dial.turn(move_right);
        dial.turn(move_left);

        assert_eq!(dial.get_position(), 55);
    }
}
