use macros::aoc_input;

fn main() {
    const INPUT: &str = aoc_input!();
    let grid = parse_input(INPUT);

    println!("Part 1: {}", solve(&grid, false));
    println!("Part 2: {}", solve(&grid, true));
}

fn parse_input(content: &str) -> Vec<&[u8]> {
    content.lines().map(str::as_bytes).collect()
}

fn solve(grid: &[&[u8]], part2: bool) -> u64 {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();

    // Use Vec instead of HashMap for performance (dense grid, small keys)
    let mut beams = vec![0u64; cols];
    let mut next_beams = vec![0u64; cols];
    let mut split_count = 0;

    // Find 'S'
    if let Some(c) = grid[0].iter().position(|&x| x == b'S') {
        beams[c] = 1;
    }

    for r in 0..rows - 1 {
        // Part 1: deduplicate beams (merge timelines)
        if !part2 {
            for count in &mut beams {
                if *count > 0 {
                    *count = 1;
                }
            }
        }

        // Reset next_beams buffer
        next_beams.fill(0);

        // Iterate over columns to update beams
        for c in 0..cols {
            let count = beams[c];
            if count == 0 {
                continue;
            }

            match grid[r + 1][c] {
                b'^' => {
                    if !part2 {
                        split_count += 1;
                    }
                    if c > 0 {
                        next_beams[c - 1] += count;
                    }
                    if c + 1 < cols {
                        next_beams[c + 1] += count;
                    }
                }
                _ => {
                    // '.' or 'S' or anything else passes through
                    next_beams[c] += count;
                }
            }
        }

        // Swap buffers for next iteration
        std::mem::swap(&mut beams, &mut next_beams);
    }

    if part2 {
        beams.iter().sum()
    } else {
        split_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_example_part1() {
        let grid = parse_input(EXAMPLE);
        assert_eq!(solve(&grid, false), 21);
    }

    #[test]
    fn test_example_part2() {
        let grid = parse_input(EXAMPLE);
        assert_eq!(solve(&grid, true), 40);
    }
}
