use macros::aoc_input;

fn main() {
    const INPUT: &str = aoc_input!();

    let mut total_part1 = 0;
    let mut total_part2 = 0;

    for line in INPUT.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let digits: Vec<u8> = line.bytes().map(|b| b - b'0').collect();

        total_part1 += solve_bank::<2>(&digits);
        total_part2 += solve_bank::<12>(&digits);
    }

    println!("Part 1 Result: {total_part1}");
    println!("Part 2 Result: {total_part2}");
}

fn solve_bank<const K: usize>(digits: &[u8]) -> u64 {
    let n = digits.len();
    let mut result = 0;
    let mut current_pos = 0;

    for remaining_needed in (0..K).rev() {
        // We can search up to a point where we still have enough digits left
        // limit is inclusive index
        let limit = n - 1 - remaining_needed;

        let mut max_digit = 0;
        let mut max_idx = current_pos;

        let slice = &digits[current_pos..=limit];

        for (idx, &digit) in slice.iter().enumerate() {
            if digit > max_digit {
                max_digit = digit;
                max_idx = current_pos + idx;
                if max_digit == 9 {
                    break; // Optimization: can't get better than 9
                }
            }
        }

        result = result * 10 + max_digit as u64;
        current_pos = max_idx + 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        let mut total_part1 = 0;

        for line in INPUT.lines() {
            let digits: Vec<u8> = line.bytes().map(|b| b - b'0').collect();
            total_part1 += solve_bank::<2>(&digits);
        }

        assert_eq!(total_part1, 357);
    }

    #[test]
    fn test_part2() {
        let mut total_part2 = 0;

        for line in INPUT.lines() {
            let digits: Vec<u8> = line.bytes().map(|b| b - b'0').collect();
            total_part2 += solve_bank::<12>(&digits);
        }

        assert_eq!(total_part2, 3121910778619);
    }
}
