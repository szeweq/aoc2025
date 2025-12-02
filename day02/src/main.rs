use macros::aoc_input;

fn main() {
    const INPUT: &str = aoc_input!();
    let ranges = parse_input(INPUT);

    let mut total_invalid_part1 = 0;
    let mut total_invalid_part2 = 0;

    for (start, end) in ranges {
        let mut n = start;
        while n <= end {
            let digits = n.ilog10() + 1;
            let next_power_of_10 = 10u64.pow(digits);
            let limit = std::cmp::min(end + 1, next_power_of_10);

            // Part 1 optimization: Skip odd digit counts
            if digits % 2 == 0 {
                for i in n..limit {
                    if is_invalid_part1(i) {
                        total_invalid_part1 += i;
                    }
                }
            }

            // Part 2 must still be checked for all numbers
            for i in n..limit {
                if is_invalid_part2(i) {
                    total_invalid_part2 += i;
                }
            }

            n = limit;
        }
    }

    println!("Part 1 Total: {total_invalid_part1}");
    println!("Part 2 Total: {total_invalid_part2}");
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').expect("Invalid range");
            let start = start.parse().expect("Invalid start number");
            let end = end.parse().expect("Invalid end number");
            (start, end)
        })
        .collect()
}

const fn is_invalid_part1(n: u64) -> bool {
    let len = n.ilog10() + 1;
    if !len.is_multiple_of(2) {
        return false;
    }

    let mid = len / 2;
    let divisor = 10u64.pow(mid);
    let first_half = n / divisor;
    let second_half = n % divisor;

    first_half == second_half
}

const fn is_invalid_part2(n: u64) -> bool {
    let len = n.ilog10() + 1;

    // Try all possible substring lengths from 1 up to len/2
    let mut l = 1;
    while l <= len / 2 {
        if len.is_multiple_of(l) {
            let repeats = len / l;
            let prefix = n / 10u64.pow(len - l);

            let mut multiplier = 0;
            let shift = 10u64.pow(l);
            let mut i = 0;
            while i < repeats {
                multiplier = multiplier * shift + 1;
                i += 1;
            }

            if prefix * multiplier == n {
                return true;
            }
        }
        l += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_part1() {
        assert!(is_invalid_part1(11));
        assert!(is_invalid_part1(22));
        assert!(is_invalid_part1(99));
        assert!(is_invalid_part1(1010));
        assert!(is_invalid_part1(1188511885));
        assert!(is_invalid_part1(222222));
        assert!(is_invalid_part1(446446));
        assert!(is_invalid_part1(38593859));

        assert!(!is_invalid_part1(101));
        assert!(!is_invalid_part1(123));
        assert!(!is_invalid_part1(12345));
    }

    #[test]
    fn test_is_invalid_part2() {
        // Part 1 examples are also valid for Part 2 (repeated 2 times)
        assert!(is_invalid_part2(11));
        assert!(is_invalid_part2(22));

        // New Part 2 examples
        assert!(is_invalid_part2(12341234)); // 2 times
        assert!(is_invalid_part2(123123123)); // 3 times
        assert!(is_invalid_part2(1212121212)); // 5 times
        assert!(is_invalid_part2(1111111)); // 7 times

        // From range examples
        assert!(is_invalid_part2(111));
        assert!(is_invalid_part2(999));
        assert!(is_invalid_part2(565656));
        assert!(is_invalid_part2(824824824));
        assert!(is_invalid_part2(2121212121));
    }

    #[test]
    fn test_example_ranges() {
        let ranges: Vec<(u64, u64)> = vec![
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
            (38593856, 38593862),
            (565653, 565659),
            (824824821, 824824827),
            (2121212118, 2121212124),
        ];

        let mut invalid_ids_part1 = Vec::new();
        let mut invalid_ids_part2 = Vec::new();

        for (start, end) in ranges {
            let mut n = start;
            while n <= end {
                let digits = n.ilog10() + 1;
                let next_power_of_10 = 10u64.pow(digits);
                let limit = std::cmp::min(end + 1, next_power_of_10);

                if digits % 2 == 0 {
                    for i in n..limit {
                        if is_invalid_part1(i) {
                            invalid_ids_part1.push(i);
                        }
                    }
                }

                for i in n..limit {
                    if is_invalid_part2(i) {
                        invalid_ids_part2.push(i);
                    }
                }
                n = limit;
            }
        }

        let expected_part1 = vec![
            11, 22,         // 11-22
            99,         // 95-115
            1010,       // 998-1012
            1188511885, // 1188511880-1188511890
            222222,     // 222220-222224
            // 1698522-1698528 none
            446446,   // 446443-446449
            38593859, // 38593856-38593862
        ];

        assert_eq!(invalid_ids_part1, expected_part1);

        let sum_part1: u64 = invalid_ids_part1.iter().sum();
        assert_eq!(sum_part1, 1227775554);

        let expected_part2 = vec![
            11, 22, // 11-22
            99, 111, // 95-115
            999, 1010,       // 998-1012
            1188511885, // 1188511880-1188511890
            222222,     // 222220-222224
            // 1698522-1698528 none
            446446,     // 446443-446449
            38593859,   // 38593856-38593862
            565656,     // 565653-565659
            824824824,  // 824824821-824824827
            2121212121, // 2121212118-2121212124
        ];

        assert_eq!(invalid_ids_part2, expected_part2);

        let sum_part2: u64 = invalid_ids_part2.iter().sum();
        assert_eq!(sum_part2, 4174379265);
    }
}
