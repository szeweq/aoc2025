use macros::aoc_input;
use std::cmp;
use std::ops::RangeInclusive;

struct Input {
    ranges: Vec<RangeInclusive<u64>>,
    available: Vec<u64>,
}

impl Input {
    fn parse(input: &str) -> Self {
        let (range_part, available_part) = input.split_once("\n\n").expect("Invalid input format");
        // Handle potential CRLF issues if split doesn't work as expected with just \n\n on windows sometimes,
        // but usually rust split handles \n fine if we normalize or just rely on the fact that \n\n is likely present.
        // Let's be robust and trim.

        let ranges = range_part
            .lines()
            .filter(|l| !l.is_empty())
            .map(|line| {
                let (start, end) = line.split_once('-').expect("Invalid range format");
                let start = start.parse().expect("Invalid start number");
                let end = end.parse().expect("Invalid end number");
                start..=end
            })
            .collect();

        let available = available_part
            .lines()
            .filter(|l| !l.is_empty())
            .map(|line| line.parse().expect("Invalid available number"))
            .collect();

        Self { ranges, available }
    }
}

fn part1(input: &Input) -> usize {
    input
        .available
        .iter()
        .filter(|&&id| input.ranges.iter().any(|r| r.contains(&id)))
        .count()
}

fn part2(input: &mut Input) -> u64 {
    let ranges = &mut input.ranges;
    ranges.sort_by_key(|r| *r.start());

    let mut merged: Vec<RangeInclusive<u64>> = Vec::new();
    if ranges.is_empty() {
        return 0;
    }

    merged.push(ranges[0].clone());

    for range in ranges.iter().skip(1) {
        let Some(last) = merged.last_mut() else {
            unreachable!()
        };
        // Check for overlap. Since we want total count of integers,
        // [1, 2] and [3, 4] are contiguous and can be merged for counting purposes?
        // Actually, the problem asks for "how many ingredient IDs".
        // [1, 2] -> 1, 2. [3, 4] -> 3, 4. Total 4.
        // Merged [1, 4] -> 1, 2, 3, 4. Total 4.
        // So yes, we can merge contiguous ranges too.
        if *range.start() <= *last.end() + 1 {
            let new_end = cmp::max(*last.end(), *range.end());
            *last = *last.start()..=new_end;
        } else {
            merged.push(range.clone());
        }
    }

    merged.iter().map(|r| r.end() - r.start() + 1).sum()
}

fn main() {
    const INPUT: &str = aoc_input!();
    // Normalize input to ensure \n\n splitting works regardless of CRLF
    let input_normalized = INPUT.replace("\r\n", "\n");
    let mut input = Input::parse(&input_normalized);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&mut input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        let input = Input::parse(EXAMPLE);
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part2() {
        let mut input = Input::parse(EXAMPLE);
        assert_eq!(part2(&mut input), 14);
    }
}
