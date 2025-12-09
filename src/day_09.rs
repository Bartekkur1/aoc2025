use std::fs::File;
use std::io::{ BufRead, BufReader };

#[derive(Clone)]
struct Position {
    x: i64,
    y: i64,
}

// Part 2 is from reddit - I gave up
fn build_polygon_segments(tiles: &[Position]) -> Vec<((i64, i64), (i64, i64))> {
    (0..tiles.len())
        .map(|i| {
            let curr = &tiles[i];
            let next = &tiles[(i + 1) % tiles.len()];
            ((curr.x, curr.y), (next.x, next.y))
        })
        .collect()
}

fn is_rectangle_inside_polygon(
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    segments: &[((i64, i64), (i64, i64))]
) -> bool {
    let (min_x, max_x) = (x1.min(x2), x1.max(x2));
    let (min_y, max_y) = (y1.min(y2), y1.max(y2));

    // Check if any polygon edge cuts through the rectangle
    for &((sx1, sy1), (sx2, sy2)) in segments {
        if sx1 == sx2 {
            // Vertical edge
            if sx1 > min_x && sx1 < max_x {
                let seg_y_min = sy1.min(sy2);
                let seg_y_max = sy1.max(sy2);
                if min_y.max(seg_y_min) < max_y.min(seg_y_max) {
                    return false;
                }
            }
        } else {
            // Horizontal edge
            if sy1 > min_y && sy1 < max_y {
                let seg_x_min = sx1.min(sx2);
                let seg_x_max = sx1.max(sx2);
                if min_x.max(seg_x_min) < max_x.min(seg_x_max) {
                    return false;
                }
            }
        }
    }

    // Use ray casting to check if rectangle center is inside polygon
    let center_x = ((min_x + max_x) as f64) / 2.0;
    let center_y = ((min_y + max_y) as f64) / 2.0;

    let mut intersections = 0;
    for &((sx1, sy1), (sx2, sy2)) in segments {
        if sx1 == sx2 {
            let edge_x = sx1 as f64;
            if edge_x > center_x {
                let edge_y_min = sy1.min(sy2) as f64;
                let edge_y_max = sy1.max(sy2) as f64;
                if center_y > edge_y_min && center_y < edge_y_max {
                    intersections += 1;
                }
            }
        }
    }

    intersections % 2 == 1
}

fn dist(p1: &Position, p2: &Position) -> i64 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn calculate_square_size(pos1: &Position, pos2: &Position) -> usize {
    let width = (pos2.x - pos1.x).abs() + 1;
    let height = (pos2.y - pos1.y).abs() + 1;
    (width * height) as usize
}

struct Solution;

impl Solution {
    pub fn read_input(file_name: &str) -> Vec<Position> {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader
            .lines()
            .map(|l| l.unwrap())
            .collect();

        let mut positions = Vec::new();
        for line in &lines {
            let cords = line.split_once(',').unwrap();
            let x = cords.0.parse::<i64>().unwrap();
            let y = cords.1.parse::<i64>().unwrap();
            positions.push(Position { x, y });
        }

        positions.reverse();
        positions
    }

    pub fn find_biggest_area(_positions: &Vec<Position>) -> usize {
        let mut all_pairs: Vec<(usize, usize, usize)> = Vec::new();
        for i in 0.._positions.len() {
            for j in 0.._positions.len() {
                if i != j {
                    all_pairs.push((dist(&_positions[i], &_positions[j]) as usize, i, j));
                }
            }
        }

        all_pairs.sort_by(|a, b| b.0.cmp(&a.0));
        all_pairs.truncate(1);

        let mut biggest = 0;
        for pair in all_pairs {
            let size = calculate_square_size(&_positions[pair.1], &_positions[pair.2]);
            if size > biggest {
                biggest = size;
            }
        }
        biggest
    }

    fn find_largest_valid_rectangle(tiles: Vec<Position>) -> usize {
        if tiles.is_empty() {
            return 0;
        }

        let mut max_area = 0;
        let segments = build_polygon_segments(&tiles);

        for i in 0..tiles.len() {
            let (x1, y1) = (tiles[i].x, tiles[i].y);
            for j in i + 1..tiles.len() {
                let (x2, y2) = (tiles[j].x, tiles[j].y);

                if x1 != x2 && y1 != y2 {
                    let width = (x2 - x1).abs() + 1;
                    let height = (y2 - y1).abs() + 1;
                    let area = (width * height) as usize;

                    if area > max_area && is_rectangle_inside_polygon(x1, y1, x2, y2, &segments) {
                        max_area = area;
                    }
                }
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_size_calc() {
        let pos1 = Position { x: 1, y: 1 };
        let pos2 = Position { x: 2, y: 2 };
        let size = calculate_square_size(&pos1, &pos2);
        println!("Calculated size: {}", size);
        assert_eq!(size, 4);
    }

    #[test]
    fn test_biggest_area() {
        let positions = Solution::read_input("./input/day_09_example.txt");
        let area = Solution::find_biggest_area(&positions);
        assert_eq!(area, 50);
    }

    #[test]
    fn test_input_read() {
        let positions = Solution::read_input("./input/day_09_example.txt");
        assert_eq!(positions.len(), 8);
    }

    #[test]
    fn test_part_1_solution() {
        let positions = Solution::read_input("./input/day_09.txt");
        let area = Solution::find_biggest_area(&positions);
        assert_eq!(area, 4755429952);
    }

    #[test]
    fn find_biggest_area_inside_example() {
        let positions = Solution::read_input("./input/day_09_example.txt");
        let area = Solution::find_largest_valid_rectangle(positions);
        assert_eq!(area, 24);
    }

    #[test]
    fn test_part_2_solution() {
        let positions = Solution::read_input("./input/day_09.txt");
        let area = Solution::find_largest_valid_rectangle(positions);
        println!("Part 2 solution: {}", area);
    }
}
