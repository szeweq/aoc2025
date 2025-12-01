use macros::aoc_input;

enum Instruction {
    Left(i32),
    Right(i32),
}

fn main() {
    const INPUT: &str = aoc_input!();
    let instructions = parse_input(INPUT);
    println!("Part 1 Result: {}", solve_part1(&instructions));
    println!("Part 2 Result: {}", solve_part2(&instructions));
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (dir, amount_str) = line.split_at(1);
            let amount: i32 = amount_str.parse().expect("Invalid number");
            match dir {
                "L" => Instruction::Left(amount),
                "R" => Instruction::Right(amount),
                _ => panic!("Invalid direction"),
            }
        })
        .collect()
}

fn solve_part1(instructions: &[Instruction]) -> usize {
    let mut current_pos = 50;
    let mut zero_count = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Left(amount) => {
                current_pos = (current_pos - amount).rem_euclid(100);
            }
            Instruction::Right(amount) => {
                current_pos = (current_pos + amount).rem_euclid(100);
            }
        }

        if current_pos == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn solve_part2(instructions: &[Instruction]) -> i32 {
    let mut current_pos: i32 = 50;
    let mut zero_count = 0;

    for instruction in instructions {
        let (amount, is_left) = match instruction {
            Instruction::Left(a) => (*a, true),
            Instruction::Right(a) => (*a, false),
        };

        let first_hit = if is_left {
            if current_pos > 0 { current_pos } else { 100 }
        } else {
            let dist = (100 - current_pos) % 100;
            if dist == 0 { 100 } else { dist }
        };

        if amount >= first_hit {
            zero_count += 1 + (amount - first_hit) / 100;
        }

        if is_left {
            current_pos = (current_pos - amount).rem_euclid(100);
        } else {
            current_pos = (current_pos + amount).rem_euclid(100);
        }
    }

    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let instructions = parse_input(input);
        assert_eq!(solve_part1(&instructions), 3);
    }

    #[test]
    fn test_example_part2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let instructions = parse_input(input);
        assert_eq!(solve_part2(&instructions), 6);
    }
}
