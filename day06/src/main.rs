use macros::aoc_input;
use std::ops::Range;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    const fn from_byte(b: u8) -> Option<Self> {
        match b {
            b'+' => Some(Self::Add),
            b'*' => Some(Self::Multiply),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<u128>,
    operator: Operator,
}

impl Problem {
    fn solve(self) -> u128 {
        let num_iter = self.numbers.into_iter();
        match self.operator {
            Operator::Add => num_iter.sum(),
            Operator::Multiply => num_iter.product(),
        }
    }
}

fn main() {
    const INPUT: &str = aoc_input!();
    let (lines, operator_line, ranges) = parse_raw_input(INPUT);

    let p1_problems = parse_part1_problems(&lines, operator_line, &ranges);
    println!("Part 1 Result: {}", solve_problems(p1_problems));

    let p2_problems = parse_part2_problems(&lines, operator_line, &ranges);
    println!("Part 2 Result: {}", solve_problems(p2_problems));
}

fn parse_raw_input(input: &str) -> (Vec<&str>, &str, Vec<Range<usize>>) {
    let mut lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return (vec![], "", vec![]);
    }

    let operator_line = lines.pop().unwrap();

    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut ranges = Vec::new();
    let mut start_col = None;

    for col in 0..max_len {
        let mut is_empty_col = true;

        for line in lines.iter() {
            let bytes = line.as_bytes();
            if col < bytes.len() && bytes[col] != b' ' {
                is_empty_col = false;
                break;
            }
        }
        if !is_empty_col {
            if start_col.is_none() {
                start_col = Some(col);
            }
        } else if let Some(start) = start_col {
            ranges.push(start..col);
            start_col = None;
        }
    }

    if let Some(start) = start_col {
        ranges.push(start..max_len);
    }

    (lines, operator_line, ranges)
}

fn parse_operator(last_line: &str, mut range: Range<usize>) -> Operator {
    let bytes = last_line.as_bytes();
    range
        .find_map(|col| {
            if col < bytes.len() {
                Operator::from_byte(bytes[col])
            } else {
                None
            }
        })
        .unwrap()
}

fn parse_part1_problems(
    lines: &[&str],
    operator_line: &str,
    ranges: &[Range<usize>],
) -> impl Iterator<Item = Problem> {
    ranges.iter().map(|range| {
        let mut numbers = Vec::new();
        // Optimization: Operator is always in the last line
        let operator = parse_operator(operator_line, range.clone());

        // Parse numbers from all lines
        for line in lines.iter() {
            let slice = if range.end <= line.len() {
                &line[range.clone()]
            } else if range.start < line.len() {
                &line[range.start..]
            } else {
                ""
            };

            let trimmed = slice.trim();
            if let Ok(num) = trimmed.parse::<u128>() {
                numbers.push(num);
            }
        }

        Problem { numbers, operator }
    })
}

fn parse_part2_problems(
    lines: &[&str],
    operator_line: &str,
    ranges: &[Range<usize>],
) -> impl Iterator<Item = Problem> {
    ranges.iter().map(|range| {
        let mut numbers = Vec::new();
        // Optimization: Operator is always in the last line
        let operator = parse_operator(operator_line, range.clone());

        let height = lines.len();
        // Iterate columns from right to left within the range
        for col in (range.start..range.end).rev() {
            let mut num_val: u128 = 0;
            let mut has_digits = false;

            // Collect digits from top rows (0 to height-2)
            for line in lines.iter().take(height) {
                let bytes = line.as_bytes();
                if col < bytes.len() {
                    let b = bytes[col];
                    if b.is_ascii_digit() {
                        num_val = num_val * 10 + (b - b'0') as u128;
                        has_digits = true;
                    }
                }
            }

            if has_digits {
                numbers.push(num_val);
            }
        }

        Problem { numbers, operator }
    })
}

fn solve_problems(problems: impl Iterator<Item = Problem>) -> u128 {
    problems.map(Problem::solve).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_example_part1() {
        let input = EXAMPLE;
        let (lines, operator_line, ranges) = parse_raw_input(input);
        let problems = parse_part1_problems(&lines, operator_line, &ranges);
        assert_eq!(solve_problems(problems), 4277556);
    }

    #[test]
    fn test_example_part2() {
        let input = EXAMPLE;
        let (lines, operator_line, ranges) = parse_raw_input(input);
        let problems = parse_part2_problems(&lines, operator_line, &ranges);
        assert_eq!(solve_problems(problems), 3263827);
    }
}
