use std::collections::HashMap;
use std::fs::File;
use std::io::{ BufRead, BufReader };

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

struct SplitterMap {
    start: Position,
    splitters: Vec<Position>,
    beams: Vec<Position>,
    beams_count: HashMap<Position, i64>,
    splits: i32,
    worlds: i32,
    map_size: Position,
}

impl SplitterMap {
    pub fn draw(&self) {
        for y in 0..self.map_size.y {
            for x in 0..self.map_size.x {
                if self.start.x == x && self.start.y == y {
                    print!("S");
                } else if self.splitters.iter().any(|s| s.x == x && s.y == y) {
                    print!("^");
                } else if self.beams_count.contains_key(&(Position { x, y })) {
                    let count = self.beams_count[&(Position { x, y })];
                    if count > 15 {
                        print!("X");
                    } else if count > 9 {
                        print!("{}", (b'A' + ((count - 10) as u8)) as char);
                    } else {
                        print!("{}", count);
                    }
                } else if self.beams.iter().any(|b| b.x == x && b.y == y) {
                    print!("|");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    pub fn shot_beam(&mut self) {
        let mut beams = vec![0i64; self.map_size.x as usize];
        beams[self.start.x as usize] = 1;

        for y in 1..self.map_size.y {
            let old = beams.clone();
            beams = vec![0i64; self.map_size.x as usize];

            for (x, &count) in old.iter().enumerate() {
                if count == 0 {
                    continue;
                }

                let is_splitter = self.splitters.iter().any(|s| s.x == (x as i32) && s.y == y);

                if is_splitter {
                    if x > 0 {
                        beams[x - 1] += count;
                    }
                    if x < beams.len() - 1 {
                        beams[x + 1] += count;
                    }
                } else {
                    beams[x] += count;
                }
            }
        }

        let result: i64 = beams.iter().sum();
        println!("P2 Result: {}", result);
    }
}

struct Solution;

impl Solution {
    pub fn build_map(file_name: &str) -> SplitterMap {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader
            .lines()
            .map(|l| l.unwrap())
            .collect();

        let mut map = SplitterMap {
            start: Position { x: 0, y: 0 },
            splitters: Vec::new(),
            beams: Vec::new(),
            beams_count: HashMap::new(),
            splits: 0,
            worlds: 0,
            map_size: Position {
                x: lines[0].len() as i32,
                y: lines.len() as i32,
            },
        };
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    'S' => {
                        map.start = Position {
                            x: x as i32,
                            y: y as i32,
                        };
                        map.beams.push(Position {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    '^' => {
                        map.splitters.push(Position {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                    _ => {}
                }
            }
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_map_draw() {
    //     let splitter_map = Solution::build_map("input/day_07_example.txt");
    //     splitter_map.draw();
    //     assert!(splitter_map.start.x == 7 && splitter_map.start.y == 0);
    // }

    // #[test]
    // fn test_shot_beam() {
    //     let mut splitter_map = Solution::build_map("input/day_07_example.txt");
    //     splitter_map.shot_beam(false);
    //     splitter_map.draw();

    //     assert!(splitter_map.splits == 21);
    // }

    // #[test]
    // fn test_solve_part_1() {
    //     let mut splitter_map = Solution::build_map("input/day_07.txt");
    //     splitter_map.shot_beam(false);
    //     // assert!(splitter_map.splits == 1594);
    // }

    #[test]
    fn test_solve_part_2_example() {
        let mut splitter_map = Solution::build_map("input/day_07_example.txt");
        splitter_map.shot_beam();
    }

    #[test]
    fn test_solve_part_2() {
        let mut splitter_map = Solution::build_map("input/day_07.txt");
        splitter_map.shot_beam();
    }
}
