use macros::{aoc_input, aoc_timed};
use std::collections::HashMap;

type DeviceID = [u8; 3];

fn parse_input(input: &str) -> HashMap<DeviceID, Vec<DeviceID>> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some((src, dests)) = line.split_once(": ") {
            let targets: Vec<DeviceID> = dests
                .split_whitespace()
                .map(|s| s.as_bytes().try_into().unwrap())
                .collect();
            graph.insert(src.as_bytes().try_into().unwrap(), targets);
        }
    }
    graph
}

fn count_paths(
    current: &DeviceID,
    target: &DeviceID,
    graph: &HashMap<DeviceID, Vec<DeviceID>>,
    memo: &mut HashMap<DeviceID, u64>,
) -> u64 {
    if current == target {
        return 1;
    }
    if let Some(&count) = memo.get(current) {
        return count;
    }

    let mut total = 0;
    if let Some(neighbors) = graph.get(current) {
        for next in neighbors {
            total += count_paths(next, target, graph, memo);
        }
    }

    memo.insert(*current, total);
    total
}

fn solve_part1(graph: &HashMap<DeviceID, Vec<DeviceID>>) -> u64 {
    let mut memo = HashMap::new();
    count_paths(b"you", b"out", graph, &mut memo)
}

fn solve_part2(graph: &HashMap<DeviceID, Vec<DeviceID>>) -> u64 {
    // Check path: svr -> dac -> fft -> out
    let mut memo = HashMap::new();
    let p1_1 = count_paths(b"svr", b"dac", graph, &mut memo);
    memo.clear();
    let p1_2 = count_paths(b"dac", b"fft", graph, &mut memo);
    memo.clear();
    let p1_3 = count_paths(b"fft", b"out", graph, &mut memo);

    let route_a = p1_1 * p1_2 * p1_3;

    // Check path: svr -> fft -> dac -> out
    memo.clear();
    let p2_1 = count_paths(b"svr", b"fft", graph, &mut memo);
    memo.clear();
    let p2_2 = count_paths(b"fft", b"dac", graph, &mut memo);
    memo.clear();
    let p2_3 = count_paths(b"dac", b"out", graph, &mut memo);

    let route_b = p2_1 * p2_2 * p2_3;

    route_a + route_b
}

#[aoc_timed]
fn main() {
    const INPUT: &str = aoc_input!();
    let graph = parse_input(INPUT);

    println!("Part 1: {}", solve_part1(&graph));
    println!("Part 2: {}", solve_part2(&graph));
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_EXAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const PART2_EXAMPLE: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part1() {
        let graph = parse_input(PART1_EXAMPLE);
        assert_eq!(solve_part1(&graph), 5);
    }

    #[test]
    fn test_part2() {
        let graph = parse_input(PART2_EXAMPLE);
        assert_eq!(solve_part2(&graph), 2);
    }
}
