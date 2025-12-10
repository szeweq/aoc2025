use macros::aoc_input;
use std::collections::{HashSet, VecDeque};

fn main() {
    const INPUT: &str = aoc_input!();
    let mut total_presses_p1 = 0;
    let mut total_presses_p2 = 0;

    for (line_idx, line) in INPUT.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let machine = parse_line(line);

        // Part 1: BFS
        if !machine.lights.is_empty() {
            match solve_part1_bfs(
                &machine.lights,
                &machine.buttons_p1_vectors(machine.lights.len()),
            ) {
                Some(p) => total_presses_p1 += p,
                None => eprintln!("Line {}: No solution for Part 1", line_idx + 1),
            }
        }

        // Part 2: ILP
        if !machine.joltage.is_empty() {
            let p2_vectors = machine.buttons_p2_vectors(machine.joltage.len());
            // Only solve if we have buttons for P2 (which we always should if parsed correctly)
            match solve_part2(machine.joltage, p2_vectors) {
                Some(p) => total_presses_p2 += p,
                None => eprintln!("Line {}: No solution for Part 2", line_idx + 1),
            }
        }
    }

    println!("Total presses Part 1: {}", total_presses_p1);
    println!("Total presses Part 2: {}", total_presses_p2);
}

struct Machine {
    lights: Vec<u8>,
    button_indices: Vec<Vec<usize>>,
    joltage: Vec<i64>,
}

impl Machine {
    fn buttons_p1_vectors(&self, size: usize) -> Vec<Vec<u8>> {
        self.button_indices
            .iter()
            .map(|indices| {
                let mut vec = vec![0u8; size];
                for &idx in indices {
                    if idx < size {
                        vec[idx] = 1;
                    }
                }
                vec
            })
            .collect()
    }

    fn buttons_p2_vectors(&self, size: usize) -> Vec<Vec<i64>> {
        self.button_indices
            .iter()
            .map(|indices| {
                let mut vec = vec![0i64; size];
                for &idx in indices {
                    if idx < size {
                        vec[idx] = 1;
                    }
                }
                vec
            })
            .collect()
    }
}

fn parse_line(line: &str) -> Machine {
    // Input format: [lights] (button1) (button2) ... {joltage}

    // Parse Lights: [ ... ]
    let lights_end = line.find(']').unwrap_or(0);
    let lights = if lights_end > 1 {
        line[1..lights_end]
            .chars()
            .map(|c| if c == '#' { 1 } else { 0 })
            .collect()
    } else {
        Vec::new()
    };

    // Parse Buttons: ( ... )
    let mut button_indices = Vec::new();
    let remaining = if lights_end < line.len() {
        &line[lights_end + 1..]
    } else {
        ""
    };

    // Find stop for buttons: start of joltage '{'
    let buttons_end = remaining.find('{').unwrap_or(remaining.len());
    let buttons_part = &remaining[..buttons_end];

    let mut offset = 0;
    while let Some(s) = buttons_part[offset..].find('(') {
        let abs_s = offset + s;
        if let Some(e) = buttons_part[abs_s..].find(')') {
            let content = &buttons_part[abs_s + 1..abs_s + e];
            let indices: Vec<usize> = if content.trim().is_empty() {
                Vec::new()
            } else {
                content
                    .split(',')
                    .map(|n| n.trim().parse().unwrap_or(0))
                    .collect()
            };
            button_indices.push(indices);
            offset = abs_s + e + 1;
        } else {
            break;
        }
    }

    // Parse Joltage: { ... }
    let joltage = if let (Some(s), Some(e)) = (remaining.find('{'), remaining.find('}')) {
        let content = &remaining[s + 1..e];
        if content.trim().is_empty() {
            Vec::new()
        } else {
            content
                .split(',')
                .map(|n| n.trim().parse().unwrap_or(0))
                .collect()
        }
    } else {
        Vec::new()
    };

    Machine {
        lights,
        button_indices,
        joltage,
    }
}

// Part 1: BFS Solver
fn solve_part1_bfs(target: &[u8], buttons: &[Vec<u8>]) -> Option<usize> {
    let num_lights = target.len();
    let start_state = vec![0u8; num_lights];

    if start_state == target {
        return Some(0);
    }

    let mut queue = VecDeque::new();
    queue.push_back((start_state.clone(), 0));

    let mut visited = HashSet::new();
    visited.insert(start_state);

    while let Some((state, depth)) = queue.pop_front() {
        if state == target {
            return Some(depth);
        }

        for button in buttons {
            let mut next_state = state.clone();
            // Apply button (XOR)
            for i in 0..num_lights {
                next_state[i] ^= button[i];
            }

            if visited.insert(next_state.clone()) {
                queue.push_back((next_state, depth + 1));
            }
        }
    }
    None
}

// Part 2: ILP Solver (Gaussian + Search)
fn solve_part2(target: Vec<i64>, buttons: Vec<Vec<i64>>) -> Option<i64> {
    let num_requirements = target.len();
    let num_buttons = buttons.len();

    // Matrix in i128 for precision. [A | b]
    let mut matrix = vec![vec![0i128; num_buttons + 1]; num_requirements];

    for r in 0..num_requirements {
        for c in 0..num_buttons {
            matrix[r][c] = buttons[c][r] as i128;
        }
        matrix[r][num_buttons] = target[r] as i128;
    }

    // Fraction-free Gaussian Elimination (Forward)
    let mut pivot_row = 0;
    let mut pivot_col_to_row = std::collections::HashMap::new();

    for c in 0..num_buttons {
        if pivot_row >= num_requirements {
            break;
        }

        // Find pivot
        let mut row = pivot_row;
        while row < num_requirements && matrix[row][c] == 0 {
            row += 1;
        }

        if row < num_requirements {
            matrix.swap(pivot_row, row);
            let pivot_val = matrix[pivot_row][c];

            for r in pivot_row + 1..num_requirements {
                if matrix[r][c] != 0 {
                    let factor = matrix[r][c];
                    for k in c..=num_buttons {
                        matrix[r][k] = matrix[r][k] * pivot_val - matrix[pivot_row][k] * factor;
                    }
                }
            }

            pivot_col_to_row.insert(c, pivot_row);
            pivot_row += 1;
        }
    }

    // Check consistency
    for r in &matrix[pivot_row..num_requirements] {
        if r[num_buttons] != 0 && r.iter().take(num_buttons).all(|&x| x == 0) {
            return None;
        }
    }

    // Identify free variables
    let free_vars: Vec<usize> = (0..num_buttons)
        .filter(|c| !pivot_col_to_row.contains_key(c))
        .collect();

    let mut min_total: Option<i64> = None;
    let mut current_free_vals = vec![0i64; free_vars.len()];

    // Heuristics for search
    fn search_int(
        idx: usize,
        free_vars: &Vec<usize>,
        free_vals: &mut Vec<i64>,
        matrix: &Vec<Vec<i128>>,
        pivot_col_to_row: &std::collections::HashMap<usize, usize>,
        num_buttons: usize,
        min_total: &mut Option<i64>,
    ) {
        if idx == free_vars.len() {
            // Check validity and calculate total
            let mut valid = true;
            let mut x = vec![0i128; num_buttons];

            for (i, &fv) in free_vars.iter().enumerate() {
                x[fv] = free_vals[i] as i128;
            }

            // Back substitution for pivots
            // Need to solve rows in correct order (effectively bottom-up of pivots)
            // Or just iterate num_pivots-1 down to 0, finding the col for that row.
            // pivot_col_to_row maps C -> R. We need to find C given R.
            let num_pivots = pivot_col_to_row.len();

            // Slow lookup? It's small.
            for r in (0..num_pivots).rev() {
                // Find pivot column for this row
                let pc = pivot_col_to_row
                    .iter()
                    .find(|&(_, &pr)| pr == r)
                    .map(|(c, _)| *c)
                    .unwrap();
                let pivot_val = matrix[r][pc];

                let mut rhs = matrix[r][num_buttons];
                for k in pc + 1..num_buttons {
                    rhs -= matrix[r][k] * x[k];
                }

                if rhs % pivot_val != 0 {
                    valid = false;
                    break;
                }

                let val = rhs / pivot_val;
                if val < 0 {
                    valid = false;
                    break;
                }
                x[pc] = val;
            }

            if valid {
                let sum: i128 = x.iter().sum();
                if sum < i64::MAX as i128 {
                    let sum_i64 = sum as i64;
                    if min_total.is_none_or(|m| sum_i64 < m) {
                        *min_total = Some(sum_i64);
                    }
                }
            }
            return;
        }

        // Search range logic
        let limit = if free_vars.len() > 1 { 200 } else { 20000 };
        for v in 0..=limit {
            free_vals[idx] = v;
            search_int(
                idx + 1,
                free_vars,
                free_vals,
                matrix,
                pivot_col_to_row,
                num_buttons,
                min_total,
            );
        }
    }

    search_int(
        0,
        &free_vars,
        &mut current_free_vals,
        &matrix,
        &pivot_col_to_row,
        num_buttons,
        &mut min_total,
    );

    min_total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example_1() {
        let target = vec![0, 1, 1, 0];
        let buttons = vec![
            vec![0, 0, 0, 1],
            vec![0, 1, 0, 1],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 1],
            vec![1, 0, 1, 0],
            vec![1, 1, 0, 0],
        ];
        assert_eq!(solve_part1_bfs(&target, &buttons), Some(2));
    }

    #[test]
    fn test_part2_example_1() {
        let target = vec![3, 5, 4, 7];
        let buttons = vec![
            vec![0, 0, 0, 1],
            vec![0, 1, 0, 1],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 1],
            vec![1, 0, 1, 0],
            vec![1, 1, 0, 0],
        ];
        assert_eq!(solve_part2(target, buttons), Some(10));
    }
}
