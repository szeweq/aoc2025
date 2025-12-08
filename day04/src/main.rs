use macros::aoc_input;

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
        let height = lines.len();
        let width = lines[0].len();
        let mut cells = Vec::with_capacity(width * height);

        for line in lines {
            for c in line.chars() {
                cells.push(match c {
                    '.' => false,
                    '@' => true,
                    _ => panic!("Invalid character in grid: {c}"),
                });
            }
        }

        Self {
            width,
            height,
            cells,
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.cells[y * self.width + x]
    }

    fn clear(&mut self, indices: &[usize]) {
        for &idx in indices {
            self.cells[idx] = false;
        }
    }

    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        const DIRS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut count = 0;
        for (dx, dy) in DIRS {
            let Some(x) = x.checked_add_signed(dx) else {
                continue;
            };
            let Some(y) = y.checked_add_signed(dy) else {
                continue;
            };
            if self.get(x, y) {
                count += 1;
            }
        }
        count
    }

    fn get_accessible_rolls(&self) -> Vec<usize> {
        let mut accessible = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) && self.count_neighbors(x, y) < 4 {
                    accessible.push(y * self.width + x);
                }
            }
        }
        accessible
    }
}

fn part1(input: &Grid) -> usize {
    input.get_accessible_rolls().len()
}

fn part2(grid: &mut Grid) -> usize {
    let mut total_removed = 0;

    loop {
        let accessible = grid.get_accessible_rolls();
        if accessible.is_empty() {
            break;
        }
        total_removed += accessible.len();
        grid.clear(&accessible);
    }
    total_removed
}

fn main() {
    const INPUT: &str = aoc_input!();
    let mut grid = Grid::parse(INPUT);
    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&mut grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        let grid = Grid::parse(EXAMPLE);
        assert_eq!(part1(&grid), 13);
    }

    #[test]
    fn test_part2() {
        let mut grid = Grid::parse(EXAMPLE);
        assert_eq!(part2(&mut grid), 43);
    }
}
