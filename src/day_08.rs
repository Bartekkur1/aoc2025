use std::collections::HashMap;
use std::fs::File;
use std::io::{ BufRead, BufReader };

#[derive(Hash, Eq, PartialEq, Clone)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    pub fn distance(&self, other: &Position) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }

    fn num_circuits(&mut self) -> usize {
        let mut roots = std::collections::HashSet::new();
        for i in 0..self.parent.len() {
            roots.insert(self.find(i));
        }
        roots.len()
    }

    fn get_circuit_sizes(&mut self) -> Vec<usize> {
        let mut sizes: HashMap<usize, usize> = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }
        let mut result: Vec<usize> = sizes.values().copied().collect();
        result.sort_by(|a, b| b.cmp(a));
        result
    }
}

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn read_input(file_name: &str) -> Vec<Position> {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader
            .lines()
            .map(|l| l.unwrap())
            .collect();

        let mut vecs: Vec<Position> = Vec::new();

        for line in lines {
            let coords: Vec<i32> = line
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            vecs.push(Position { x: coords[0], y: coords[1], z: coords[2] });
        }

        vecs
    }

    pub fn solve(positions: &Vec<Position>, num_connections: usize) -> usize {
        let n = positions.len();

        let mut edges: Vec<(f64, usize, usize)> = Vec::new();
        for i in 0..n {
            for j in i + 1..n {
                let dist_sq = positions[i].distance(&positions[j]);
                edges.push((dist_sq, i, j));
            }
        }

        edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mut uf = UnionFind::new(n);
        let mut attempts = 0;
        for &(_, i, j) in &edges {
            if attempts >= num_connections {
                break;
            }
            uf.union(i, j);
            attempts += 1;
        }

        let circuit_sizes = uf.get_circuit_sizes();

        for &(_, i, j) in &edges {
            if uf.union(i, j) {
                if uf.num_circuits() == 1 {
                    println!(
                        "Final connection: ({}, {}, {}) to ({}, {}, {})",
                        positions[i].x,
                        positions[i].y,
                        positions[i].z,
                        positions[j].x,
                        positions[j].y,
                        positions[j].z
                    );
                    println!("Result: {}", positions[i].x * positions[j].x);
                }
            }
        }

        circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn read_example_input() {
        let positions = Solution::read_input("./input/day_08_example.txt");
        assert!(positions.len() == 20);
    }

    #[test]
    fn solve_example() {
        let positions = Solution::read_input("./input/day_08_example.txt");
        let res = Solution::solve(&positions, 10);
        assert_eq!(res, 40);
    }

    #[test]
    fn solve() {
        let positions = Solution::read_input("./input/day_08.txt");
        let res = Solution::solve(&positions, 1000);
        assert_eq!(res, 102816);
    }
}
