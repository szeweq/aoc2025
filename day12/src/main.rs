use macros::aoc_input;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Shape {
    id: usize,
    points: Vec<Point>,
    width: usize,
    height: usize,
}

type Point = (i32, i32);

impl Shape {
    fn normalize(&mut self) {
        if self.points.is_empty() {
            return;
        }
        let min_r = self.points.iter().map(|p| p.0).min().unwrap();
        let min_c = self.points.iter().map(|p| p.1).min().unwrap();
        for p in &mut self.points {
            p.0 -= min_r;
            p.1 -= min_c;
        }
        // Recalculate dims
        self.height = (self.points.iter().map(|p| p.0).max().unwrap() + 1) as usize;
        self.width = (self.points.iter().map(|p| p.1).max().unwrap() + 1) as usize;

        // Sort points for consistent comparison
        self.points
            .sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    }

    fn rotate(&self) -> Shape {
        let mut new_points = Vec::new();
        for p in &self.points {
            // Rotate 90 deg clockwise: (r, c) -> (c, -r)
            new_points.push((p.1, -p.0));
        }
        let mut s = Shape {
            id: self.id,
            points: new_points,
            height: 0,
            width: 0,
        };
        s.normalize();
        s
    }

    fn flip(&self) -> Shape {
        let mut new_points = Vec::new();
        for p in &self.points {
            // Flip horizontal: (r, c) -> (r, -c)
            new_points.push((p.0, -p.1));
        }
        let mut s = Shape {
            id: self.id,
            points: new_points,
            height: 0,
            width: 0,
        };
        s.normalize();
        s
    }
}

// Generate all 8 orientations (or fewer if symmetric)
fn generate_orientations(base: &Shape) -> Vec<Shape> {
    let mut distinct = HashSet::new();
    let mut results = Vec::new();

    let mut curr = base.clone();
    // 4 rotations
    for _ in 0..4 {
        let key = curr.points.clone();
        if distinct.insert(key) {
            results.push(curr.clone());
        }

        // Flip
        let flipped = curr.flip();
        let key_flip = flipped.points.clone();
        if distinct.insert(key_flip) {
            results.push(flipped);
        }

        curr = curr.rotate();
    }
    results
}

struct Region {
    width: usize,
    height: usize,
    requirements: Vec<usize>, // List of shape IDs to place
}

fn parse_input(input: &str) -> (HashMap<usize, Vec<Shape>>, Vec<Region>) {
    let mut shapes_map = HashMap::new();
    let mut regions = Vec::new();

    let mut lines = input.lines().peekable();

    while let Some(line) = lines.peek() {
        let line = line.trim();
        if line.is_empty() {
            lines.next();
            continue;
        }

        // Check for Region line: "WxH:"
        if line.contains('x') && line.contains(':') {
            // This is a region line.
            // Since Example has regions at the end, we can switch to parsing regions?
            // Or just parse it here.
            let line = lines.next().unwrap();
            let (dims, counts) = line.split_once(':').unwrap();
            let (w_s, h_s) = dims.split_once('x').unwrap();
            let w: usize = w_s.trim().parse().unwrap();
            let h: usize = h_s.trim().parse().unwrap();

            let mut requirements = Vec::new();
            let count_parts: Vec<usize> = counts
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            for (id, &cnt) in count_parts.iter().enumerate() {
                for _ in 0..cnt {
                    requirements.push(id);
                }
            }
            regions.push(Region {
                width: w,
                height: h,
                requirements,
            });
            continue;
        }

        // Check for Shape header: "ID:"
        if let Some((id_str, _)) = line.split_once(':')
            && let Ok(id) = id_str.trim().parse::<usize>()
        {
            // Shape header
            lines.next(); // Consume header
            let mut points = Vec::new();
            let mut r = 0;
            while let Some(shape_line) = lines.peek() {
                if shape_line.trim().is_empty() || shape_line.contains(':') {
                    break;
                }

                let shape_line = lines.next().unwrap();
                for (c_idx, char) in shape_line.chars().enumerate() {
                    if char == '#' {
                        points.push((r, c_idx as i32));
                    }
                }
                r += 1;
            }

            let mut shape = Shape {
                id,
                points,
                width: 0,
                height: 0,
            };
            shape.normalize();
            shapes_map.insert(id, generate_orientations(&shape));
            continue;
        }

        // If neither, consume line (shouldn't happen if format is strict)
        lines.next();
    }

    (shapes_map, regions)
}

// Bitmask grid for performance. Up to 64 width.
struct BitGrid {
    rows: Vec<u64>,
    width: usize,
    height: usize,
}

impl BitGrid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            rows: vec![0; height],
            width,
            height,
        }
    }

    fn can_place(&self, shape: &Shape, top_left_r: i32, top_left_c: i32) -> bool {
        // Bounds check
        if top_left_r < 0 || top_left_c < 0 {
            return false;
        }
        if top_left_r + shape.height as i32 > self.height as i32 {
            return false;
        }
        if top_left_c + shape.width as i32 > self.width as i32 {
            return false;
        }

        for p in &shape.points {
            let r = (top_left_r + p.0) as usize;
            let c = (top_left_c + p.1) as usize;
            if (self.rows[r] >> c) & 1 == 1 {
                return false;
            }
        }
        true
    }

    fn place(&mut self, shape: &Shape, top_left_r: i32, top_left_c: i32) {
        for p in &shape.points {
            let r = (top_left_r + p.0) as usize;
            let c = (top_left_c + p.1) as usize;
            self.rows[r] |= 1 << c;
        }
    }

    fn remove(&mut self, shape: &Shape, top_left_r: i32, top_left_c: i32) {
        for p in &shape.points {
            let r = (top_left_r + p.0) as usize;
            let c = (top_left_c + p.1) as usize;
            self.rows[r] &= !(1 << c);
        }
    }

    fn first_empty(&self, start_r: usize) -> Option<(usize, usize)> {
        for r in start_r..self.height {
            let row = self.rows[r];
            // We want to find the first 0 bit.
            // Invert the row so 0s become 1s.
            // We are only interested in bits 0..width.
            // Mask out high bits.
            let mask = if self.width == 64 {
                !0
            } else {
                (1u64 << self.width) - 1
            };
            let inverted = (!row) & mask;
            if inverted != 0 {
                let c = inverted.trailing_zeros() as usize;
                return Some((r, c));
            }
        }
        None
    }

    fn copy_from(&mut self, other: &BitGrid) {
        self.rows.copy_from_slice(&other.rows);
    }
}
// first_empty removed because built into recursion logic

fn solve_part1(shapes_map: &HashMap<usize, Vec<Shape>>, regions: &[Region]) -> usize {
    let mut count = 0;

    // Pre-calculate shape areas for spacer calculation
    let mut shape_areas = HashMap::new();
    let mut min_shape_area = usize::MAX;
    for (&id, variants) in shapes_map {
        let area = variants[0].points.len();
        shape_areas.insert(id, area);
        if area < min_shape_area {
            min_shape_area = area;
        }
    }

    if min_shape_area == usize::MAX {
        min_shape_area = 0;
    }

    for (idx, region) in regions.iter().enumerate() {
        let mut grid = BitGrid::new(region.width, region.height);

        // Count occurrences of each shape
        let max_id = *shapes_map.keys().max().unwrap_or(&0);
        let mut counts = vec![0; max_id + 1];
        let mut total_shape_area = 0;

        for &id in &region.requirements {
            counts[id] += 1;
            total_shape_area += shape_areas[&id];
        }

        // Calculate spacers needed
        let grid_area = region.width * region.height;
        if total_shape_area > grid_area {
            println!(
                "Region {} Impossible: Area {} > Grid {}",
                idx, total_shape_area, grid_area
            );

            continue;
        }
        let mut spacers = grid_area - total_shape_area;

        println!(
            "Region {} ({}x{}): Solving with {} spacers. Shapes Area: {}",
            idx, region.width, region.height, spacers, total_shape_area
        );

        let mut visited_scratch = BitGrid::new(region.width, region.height);

        if solve_exact_cover(
            &mut grid,
            &mut counts,
            &shape_areas,
            shapes_map,
            &mut spacers,
            total_shape_area,
            min_shape_area,
            0,
            &mut visited_scratch,
        ) {
            println!("Region {} Solved!", idx);
            count += 1;
        } else {
            println!("Region {} Failed", idx);
        }
    }
    count
}

fn solve_exact_cover(
    grid: &mut BitGrid,
    counts: &mut [usize],
    shape_areas: &HashMap<usize, usize>,
    shapes_map: &HashMap<usize, Vec<Shape>>,
    spacers: &mut usize,
    required_area: usize,
    min_shape_area: usize,
    start_r: usize,
    visited_scratch: &mut BitGrid,
) -> bool {
    let (r, c) = match grid.first_empty(start_r) {
        Some(pos) => pos,
        None => return true,
    };

    // Pruning: Flood Fill check for dead space
    // Only perform this check if we have spacers, because pure tiling (0 spacers)
    // implicitly checks area locally. But having spacers allows fragmentation.
    // Optimization: Don't check at every depth?
    // Let's check always for now. Grid is small (50x50), BFS is fast.
    // Pruning: Flood Fill check for dead space
    if *spacers > 0 && prune_dead_space(grid, required_area, min_shape_area, visited_scratch) {
        return false;
    }

    // Option 1: Place a shape (Priority over spacer to fill Area)
    for id in 0..counts.len() {
        if counts[id] > 0 {
            counts[id] -= 1;
            let area = shape_areas[&id];

            let variants = &shapes_map[&id];
            for variant in variants {
                // Optimization: Only variants that cover (r, c) with their first point
                // variant.points[0] is (p0_r, p0_c).
                let p0 = &variant.points[0];
                let top_left_r = r as i32 - p0.0;
                let top_left_c = c as i32 - p0.1;

                if grid.can_place(variant, top_left_r, top_left_c) {
                    grid.place(variant, top_left_r, top_left_c);

                    // Decrement required area
                    if solve_exact_cover(
                        grid,
                        counts,
                        shape_areas,
                        shapes_map,
                        spacers,
                        required_area - area,
                        min_shape_area,
                        r,
                        visited_scratch,
                    ) {
                        return true;
                    }
                    grid.remove(variant, top_left_r, top_left_c);
                }
            }

            counts[id] += 1;
        }
    }

    // Option 2: Place a spacer (1x1)
    // Only if shapes failed (or we are branching? No, logic is "Try shapes", if fail, "Try spacer").
    // Wait, shapes might fit later?
    // "Try shapes at (r,c)" covers all possibilities where (r,c) is covered by a shape.
    // "Place spacer at (r,c)" covers the possibility where (r,c) is NOT covered by a shape.
    // These are exhaustive.
    if *spacers > 0 {
        *spacers -= 1;
        grid.rows[r] |= 1 << c;
        if solve_exact_cover(
            grid,
            counts,
            shape_areas,
            shapes_map,
            spacers,
            required_area,
            min_shape_area,
            r,
            visited_scratch,
        ) {
            return true;
        }
        grid.rows[r] &= !(1 << c);
        *spacers += 1;
    }

    false
}

fn prune_dead_space(
    grid: &BitGrid,
    required_area: usize,
    min_shape_area: usize,
    visited: &mut BitGrid,
) -> bool {
    // If we have very small requirement, maybe not worth pruning?
    if required_area == 0 {
        return false;
    }

    // Use passed-in scratch buffer
    visited.copy_from(grid);

    let mut total_usable_area = 0;

    for r in 0..grid.height {
        // Optimization: Skip full rows
        let mask = if grid.width == 64 {
            !0
        } else {
            (1u64 << grid.width) - 1
        };
        if (visited.rows[r] & mask) == mask {
            continue;
        }

        for c in 0..grid.width {
            if (visited.rows[r] >> c) & 1 == 0 {
                // Found empty cell, start BFS
                let mut q = VecDeque::new();
                q.push_back((r, c));
                visited.rows[r] |= 1 << c;
                let mut component_size = 0;

                while let Some((curr_r, curr_c)) = q.pop_front() {
                    component_size += 1;

                    // Neighbors
                    let deltas = [(0, 1), (0, -1), (1, 0), (-1, 0)];
                    for (dr, dc) in deltas {
                        let nr = curr_r as i32 + dr;
                        let nc = curr_c as i32 + dc;
                        if nr >= 0 && nr < grid.height as i32 && nc >= 0 && nc < grid.width as i32 {
                            let nr = nr as usize;
                            let nc = nc as usize;
                            if (visited.rows[nr] >> nc) & 1 == 0 {
                                visited.rows[nr] |= 1 << nc;
                                q.push_back((nr, nc));
                            }
                        }
                    }
                }

                // If this component is big enough to hold at least the smallest shape, count it.
                // Otherwise it's dead space (e.g. size 1 or 2 when min shape is 4).
                // Spacers can fill dead space, BUT we want to know if USABLE space is enough for shapes.
                // Wait. Spacers can fill *ANY* space.
                // So "Dead Space" concept only applies if ComponentSize < MinShapeSize AND ComponentSize > Spacers?
                // No.
                // A component of size S. If S < MinShape, we MUST fill it with Spacers.
                // If Spacers < S, we can't fill it. Impossible.
                // BUT: We don't track per-component spacer usage here easily.
                // Simpler checks:
                // 1. Total area of (Components >= MinShape) must be >= RequiredArea?
                //    No, because spacers can contribute to large components too but usually we save them.
                //    Actually, if we assume worst case: perfectly pack shapes into usable components.
                //    If `Sum(Components >= MinShape)` < RequiredArea, then we HAVE to put shapes into small components.
                //    But shapes don't fit into small components!
                //    So YES: `Sum(Components >= MinShape)` MUST be >= RequiredArea.
                if component_size >= min_shape_area {
                    total_usable_area += component_size;
                }
            }
        }
    }

    total_usable_area < required_area
}

fn main() {
    let input = aoc_input!();
    let (shapes, regions) = parse_input(input);

    println!("Part 1: {}", solve_part1(&shapes, &regions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_part1() {
        let (shapes, regions) = parse_input(EXAMPLE);
        assert_eq!(solve_part1(&shapes, &regions), 2);
    }
}
