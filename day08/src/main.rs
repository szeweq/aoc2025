use macros::aoc_input;
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use std::cmp::Reverse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pt {
    x: i64,
    y: i64,
    z: i64,
}

impl Pt {
    const fn dist_sq(&self, other: &Self) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

struct UnionFind {
    nodes: Box<[(usize, usize)]>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            nodes: (0..n).map(|i| (i, 1)).collect(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.nodes[i].0 == i {
            i
        } else {
            let root = self.find(self.nodes[i].0);
            self.nodes[i].0 = root;
            root
        }
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i == root_j {
            false
        } else {
            if self.nodes[root_i].1 < self.nodes[root_j].1 {
                self.nodes[root_i].0 = root_j;
                self.nodes[root_j].1 += self.nodes[root_i].1;
            } else {
                self.nodes[root_j].0 = root_i;
                self.nodes[root_i].1 += self.nodes[root_j].1;
            }
            true
        }
    }
}

fn main() {
    const INPUT: &str = aoc_input!();
    let points = parse_input(INPUT);

    println!("Part 1: {}", solve_part1(&points, 1000));
    println!("Part 2: {}", solve_part2(&points));
}

fn parse_input(content: &str) -> Vec<Pt> {
    content
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            let z = parts.next().unwrap().parse().unwrap();
            Pt { x, y, z }
        })
        .collect()
}

#[derive(Clone, Copy)]
struct Pair {
    u: usize,
    v: usize,
    dist_sq: i64,
}

#[cfg(feature = "rayon")]
fn get_sorted_pairs(points: &[Pt]) -> Vec<Pair> {
    let mut pairs: Vec<Pair> = (0..points.len())
        .into_par_iter()
        .flat_map(|i| {
            (i + 1..points.len()).into_par_iter().map(move |j| Pair {
                u: i,
                v: j,
                dist_sq: points[i].dist_sq(&points[j]),
            })
        })
        .collect();

    pairs.par_sort_unstable_by_key(|p| p.dist_sq);
    pairs
}

#[cfg(not(feature = "rayon"))]
fn get_sorted_pairs(points: &[Pt]) -> Vec<Pair> {
    let mut pairs = Vec::with_capacity(points.len() * (points.len() - 1) / 2);
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            pairs.push(Pair {
                u: i,
                v: j,
                dist_sq: points[i].dist_sq(&points[j]),
            });
        }
    }
    pairs.sort_unstable_by_key(|p| p.dist_sq);
    pairs
}

fn solve_part1(points: &[Pt], connections: usize) -> usize {
    let pairs = get_sorted_pairs(points);
    let mut uf = UnionFind::new(points.len());

    for pair in pairs.iter().take(connections.min(pairs.len())) {
        uf.union(pair.u, pair.v);
    }

    let mut sizes = Vec::new();
    let mut visited_roots = std::collections::HashSet::new();

    for i in 0..points.len() {
        let root = uf.find(i);
        if visited_roots.insert(root) {
            sizes.push(uf.nodes[root].1);
        }
    }

    sizes.sort_by_key(|&s| Reverse(s));

    if sizes.len() < 3 {
        sizes.iter().product()
    } else {
        sizes[0] * sizes[1] * sizes[2]
    }
}

fn solve_part2(points: &[Pt]) -> i64 {
    let pairs = get_sorted_pairs(points);
    let mut uf = UnionFind::new(points.len());
    let mut components = points.len();

    for pair in pairs {
        if uf.union(pair.u, pair.v) {
            components -= 1;
            if components == 1 {
                return points[pair.u].x * points[pair.v].x;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_example_part1() {
        let points = parse_input(EXAMPLE);
        assert_eq!(solve_part1(&points, 10), 40);
    }

    #[test]
    fn test_example_part2() {
        let points = parse_input(EXAMPLE);
        assert_eq!(solve_part2(&points), 25272);
    }
}
