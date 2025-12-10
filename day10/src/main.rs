use macros::aoc_input;

fn main() {
    const INPUT: &str = aoc_input!();
    let mut total_presses_p1 = 0;
    let mut total_presses_p2 = 0;

    for (line_idx, line) in INPUT.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        // Parse Lights (P1 Target) [ # . # ]
        let lights_start = match line.find('[') {
            Some(i) => i,
            None => continue,
        };
        let lights_end = line.find(']').expect("Missing closing bracket for lights");
        let lights_str = &line[lights_start + 1..lights_end];
        let target_p1: Vec<u8> = lights_str
            .chars()
            .map(|c| if c == '#' { 1 } else { 0 })
            .collect();
        let num_lights = target_p1.len();

        let mut raw_button_indices: Vec<Vec<usize>> = Vec::new();
        let mut remaining = &line[lights_end + 1..];

        // Parse Buttons (...)
        while let Some(start) = remaining.find('(') {
            let end_offset = remaining[start..]
                .find(')')
                .expect("Missing closing parenthesis for button");
            let end = start + end_offset;
            let content = &remaining[start + 1..end];

            let indices: Vec<usize> = if content.trim().is_empty() {
                Vec::new()
            } else {
                content
                    .split(',')
                    .map(|s| s.trim().parse().expect("Invalid number in button config"))
                    .collect()
            };
            raw_button_indices.push(indices);

            remaining = &remaining[end + 1..];
            if remaining.trim_start().starts_with('{') {
                break;
            }
        }

        // Parse Joltage (P2 Target) { 1, 2, 3 }
        let target_p2: Vec<i64> = remaining.find('{').map_or_else(Vec::new, |brace_start| {
            let brace_end = remaining[brace_start..]
                .find('}')
                .expect("Missing closing brace for joltage requirements");
            let joltage_str = &remaining[brace_start + 1..brace_start + brace_end];
            if joltage_str.trim().is_empty() {
                Vec::new()
            } else {
                joltage_str
                    .split(',')
                    .map(|s| {
                        s.trim()
                            .parse()
                            .expect("Invalid number in joltage requirements")
                    })
                    .collect()
            }
        });
        let num_joltage_reqs = target_p2.len();

        // Construct P1 Buttons
        let mut buttons_p1 = Vec::new();
        for indices in &raw_button_indices {
            let mut button_vec = vec![0u8; num_lights];
            for &idx in indices {
                if idx < num_lights {
                    button_vec[idx] = 1;
                }
            }
            buttons_p1.push(button_vec);
        }

        // Construct P2 Buttons
        let mut buttons_p2 = Vec::new();
        if num_joltage_reqs > 0 {
            for indices in &raw_button_indices {
                let mut button_vec = vec![0i64; num_joltage_reqs];
                for &idx in indices {
                    if idx < num_joltage_reqs {
                        button_vec[idx] = 1;
                    }
                }
                buttons_p2.push(button_vec);
            }
        }

        // Solve Part 1
        match solve(target_p1, buttons_p1) {
            Some(presses) => {
                total_presses_p1 += presses;
            }
            None => {
                eprintln!("Line {}: No solution found for Part 1", line_idx + 1);
            }
        }

        // Solve Part 2
        if !target_p2.is_empty() {
            match solve_part2(target_p2, buttons_p2) {
                Some(presses) => {
                    // println!("Line {}: P2 Presses = {}", line_idx + 1, presses);
                    total_presses_p2 += presses;
                }
                None => {
                    // println!("Line {}: No solution found for Part 2", line_idx + 1);
                    eprintln!("Line {}: No solution found for Part 2", line_idx + 1);
                }
            }
        }
    }

    println!("Total presses Part 1: {}", total_presses_p1);
    println!("Total presses Part 2: {}", total_presses_p2);
}

fn solve(target: Vec<u8>, buttons: Vec<Vec<u8>>) -> Option<usize> {
    let num_lights = target.len(); // Rows
    let num_buttons = buttons.len(); // Cols

    // Augmented matrix [A | b]
    // A maps button (col) to light effect (row)
    // equation: sum(x_j * button_j[i]) = target[i]

    let mut matrix = vec![vec![0u8; num_buttons + 1]; num_lights];

    for r in 0..num_lights {
        for c in 0..num_buttons {
            matrix[r][c] = buttons[c][r];
        }
        matrix[r][num_buttons] = target[r];
    }

    // Gaussian elimination (RREF)
    let mut pivot_row = 0;
    // pivot_cols removed as it was unused
    let mut pivot_col_to_row = std::collections::HashMap::new();

    for c in 0..num_buttons {
        if pivot_row >= num_lights {
            break;
        }

        // Find pivot in current column `c` at or below `pivot_row`
        let mut row = pivot_row;
        while row < num_lights && matrix[row][c] == 0 {
            row += 1;
        }

        if row < num_lights {
            // Swap pivot row into place
            matrix.swap(pivot_row, row);

            // Eliminate other rows
            for r in 0..num_lights {
                if r != pivot_row && matrix[r][c] == 1 {
                    for k in c..=num_buttons {
                        matrix[r][k] ^= matrix[pivot_row][k];
                    }
                }
            }

            pivot_col_to_row.insert(c, pivot_row);
            pivot_row += 1;
        }
    }

    // Check for consistency
    // Any row starting with zeros must have a zero in the augmented column
    for r in &matrix[pivot_row..num_lights] {
        if r[num_buttons] == 1 {
            return None; // 0 = 1, impossible
        }
    }

    // Minimize Hamming weight (Part 1 logic)
    let free_vars: Vec<usize> = (0..num_buttons)
        .filter(|c| !pivot_col_to_row.contains_key(c))
        .collect();

    let num_free = free_vars.len();
    if num_free > 20 {
        // eprintln!("Warning: {} free variables. Brute force may be slow.", num_free);
    }

    let mut min_weight = usize::MAX;

    for i in 0..(1u64 << num_free) {
        let mut x = vec![0u8; num_buttons];

        // Set free variables
        for (bit_idx, &var_idx) in free_vars.iter().enumerate() {
            if (i >> bit_idx) & 1 == 1 {
                x[var_idx] = 1;
            }
        }

        for (&pc, &pr) in &pivot_col_to_row {
            let mut val = matrix[pr][num_buttons];
            for &fv in &free_vars {
                if matrix[pr][fv] == 1 {
                    val ^= x[fv];
                }
            }
            x[pc] = val;
        }

        let weight = x.iter().filter(|&&v| v == 1).count();
        if weight < min_weight {
            min_weight = weight;
        }
    }

    Some(min_weight)
}

fn solve_part2(target: Vec<i64>, buttons: Vec<Vec<i64>>) -> Option<i64> {
    let num_requirements = target.len();
    let num_buttons = buttons.len();

    // Matrix in i128 for precision.
    // [A | b]
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

            // Eliminate other rows (Forward only for now, we'll backsubstitute)
            // To zero out matrix[r][c] using pivot matrix[p][c]:
            // Row_r = Row_r * Pivot - Row_p * leading_val_r
            // But strict fraction free might grow numbers too fast.
            // Since coefficients are 0/1 initially, growth is likely manageable with i128.

            let pivot_val = matrix[pivot_row][c];

            for r in pivot_row + 1..num_requirements {
                if matrix[r][c] != 0 {
                    let factor = matrix[r][c];
                    // We need to operate on the whole row.
                    // r_new = r * pivot - p * factor
                    // This ensures r_new[c] = factor * pivot - pivot * factor = 0

                    for k in c..=num_buttons {
                        matrix[r][k] = matrix[r][k] * pivot_val - matrix[pivot_row][k] * factor;
                    }

                    // Optional reduction to keep numbers small?
                    // gcd of row?
                }
            }

            pivot_col_to_row.insert(c, pivot_row);
            pivot_row += 1;
        }
    }

    // Check for consistency
    // Rows from pivot_row onwards should be all zeros (including RHS)
    for r in &matrix[pivot_row..num_requirements] {
        if r[num_buttons] != 0 {
            // Check if LHS is all zero
            let all_zero = r.iter().all(|&x| x == 0);
            if all_zero {
                return None; // 0 != non-zero
            }
        }
    }

    // Back substitution?
    // Or just solve using the pivots.
    // It's not fully diagonalized.
    // We have a triangular matrix now.

    // Identify free variables
    let free_vars: Vec<usize> = (0..num_buttons)
        .filter(|c| !pivot_col_to_row.contains_key(c))
        .collect();

    // Search?
    // Equation for pivot `c` at `row`:
    // pivot_val * x[c] + sum(matrix[row][k] * x[k]) = matrix[row][num_buttons]  (for k > c)
    // x[c] = (RHS - sum(...)) / pivot_val
    // Must be divisible.

    if free_vars.len() > 8 {
        // With i128, exact check is fast, but brute force is limited by 2^N.
        // If N > 8, might be slow if we search large space.
    }

    let mut min_total: Option<i64> = None;
    let mut current_free_vals = vec![0i64; free_vars.len()]; // Using i64 for search values

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

            // We need to resolve all variables (x) to compute total.
            // We can resolve from last pivot row up to first.
            let mut x = vec![0i128; num_buttons];

            // Set free vars
            for (i, &fv) in free_vars.iter().enumerate() {
                x[fv] = free_vals[i] as i128;
            }

            // Back subst
            // Since pivot map is arbitrary (columns), we need to iterate rows from bottom up in the pivot set.
            // But `pivot_col_to_row` maps col -> row.
            // We want to find which col is the pivot for row `r`.
            // Let's invert map or just iterate rows `num_pivots-1` down to 0.
            let mut row_to_pivot_col = std::collections::HashMap::new();
            for (&c, &r) in pivot_col_to_row {
                row_to_pivot_col.insert(r, c);
            }

            let num_pivots = pivot_col_to_row.len();
            for r in (0..num_pivots).rev() {
                let pc = row_to_pivot_col[&r];
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
                // Check i64 limits if needed, usually fine.
                if sum < i64::MAX as i128 {
                    let sum_i64 = sum as i64;
                    if min_total.is_none() || sum_i64 < min_total.unwrap() {
                        *min_total = Some(sum_i64);
                    }
                }
            }
            return;
        }
        // Search range
        // For integers, if there's no unique solution, the solution space is a lattice.
        // We want minimal non-negative solution.
        // Heuristic: Check small numbers.
        // Single free variable might need large value to balance large constraints.
        // Multiple free variables explode complexity, so we keep them smaller.
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
    fn test_example_1() {
        // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1)
        // Lights: 0=off, 1=on. Target: .##. -> [0, 1, 1, 0]
        // Buttons:
        // 0: (3) -> [0,0,0,1]
        // 1: (1,3) -> [0,1,0,1]
        // 2: (2) -> [0,0,1,0]
        // 3: (2,3) -> [0,0,1,1]
        // 4: (0,2) -> [1,0,1,0]
        // 5: (0,1) -> [1,1,0,0]

        let target = vec![0, 1, 1, 0];
        let buttons = vec![
            vec![0, 0, 0, 1],
            vec![0, 1, 0, 1],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 1],
            vec![1, 0, 1, 0],
            vec![1, 1, 0, 0],
        ];

        assert_eq!(solve(target, buttons), Some(2));
    }

    #[test]
    fn test_example_2() {
        // [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4)
        // Target: [0,0,0,1,0]
        let target = vec![0, 0, 0, 1, 0];
        let buttons = vec![
            vec![1, 0, 1, 1, 1],
            vec![0, 0, 1, 1, 0],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 0, 0],
            vec![0, 1, 1, 1, 1],
        ];
        // Expected: 3
        assert_eq!(solve(target, buttons), Some(3));
    }

    #[test]
    fn test_example_3() {
        // [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2)
        // Target: .###.# -> [0,1,1,1,0,1]
        let target = vec![0, 1, 1, 1, 0, 1];
        let buttons = vec![
            vec![1, 1, 1, 1, 1, 0],
            vec![1, 0, 0, 1, 1, 0],
            vec![1, 1, 1, 0, 1, 1],
            vec![0, 1, 1, 0, 0, 0],
        ];
        // Expected: 2 (buttons 0,3,4 and 0,1,2,4,5 -> indices 1 and 2 in 0-indexed list? No, example says "pressing buttons (0,3,4) and (0,1,2,4,5)")
        // The list in example text: (0,1,2,3,4) is button 1 [idx 0]
        // (0,3,4) is button 2 [idx 1]
        // (0,1,2,4,5) is button 3 [idx 2]
        // (1,2) is button 4 [idx 3]
        // "pressing buttons (0,3,4) and (0,1,2,4,5)" means idx 1 and idx 2.
        assert_eq!(solve(target, buttons), Some(2));
    }

    #[test]
    fn test_part2_example_1() {
        // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        let target = vec![3, 5, 4, 7];
        let buttons = vec![
            vec![0, 0, 0, 1],
            vec![0, 1, 0, 1],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 1],
            vec![1, 0, 1, 0],
            vec![1, 1, 0, 0],
        ];
        // Expected: 10
        assert_eq!(solve_part2(target, buttons), Some(10));
    }

    #[test]
    fn test_part2_example_2() {
        // [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        let target = vec![7, 5, 12, 7, 2];
        let buttons = vec![
            vec![1, 0, 1, 1, 1],
            vec![0, 0, 1, 1, 0],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 0, 0],
            vec![0, 1, 1, 1, 1],
        ];
        // Expected: 12
        assert_eq!(solve_part2(target, buttons), Some(12));
    }

    #[test]
    fn test_part2_example_3() {
        // [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        let target = vec![10, 11, 11, 5, 10, 5];
        let buttons = vec![
            vec![1, 1, 1, 1, 1, 0],
            vec![1, 0, 0, 1, 1, 0],
            vec![1, 1, 1, 0, 1, 1],
            vec![0, 1, 1, 0, 0, 0],
        ];
        // Expected: 11
        assert_eq!(solve_part2(target, buttons), Some(11));
    }
}
