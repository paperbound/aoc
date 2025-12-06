use raoc2025::utils::*;
use std::ops::RangeInclusive;

fn main() {
    let input = read_to_string("inputs/day05.txt");
    let answer1 = part1(&input);
    let answer2 = part2(&input);

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn part1(input: &str) -> u64 {
    let mut count = 0;
    let mut fresh: Vec<RangeInclusive<u64>> = Vec::new();
    let mut lines = input.lines();
    for line in lines.by_ref() {
        let range = match line.split_once('-') {
            Some((begin, end)) => begin.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap(),
            None => break,
        };
        fresh.push(range);
    }
    for line in lines {
        let ingredient = line.parse::<u64>().unwrap();
        for range in &fresh {
            if range.contains(&ingredient) {
                count += 1;
                break;
            }
        }
    }
    count
}

fn part2(input: &str) -> usize {
    let mut count = 0;
    let mut fresh: Vec<RangeInclusive<usize>> = Vec::new();
    let mut lines = input.lines();
    for line in lines.by_ref() {
        let range = match line.split_once('-') {
            Some((begin, end)) => begin.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap(),
            None => break,
        };
        fresh.push(range);
    }
    fresh.sort_by_key(|r| *r.start());
    let mut start = *fresh[0].start();
    let mut end = *fresh[0].end();
    for range in fresh.iter().skip(1) {
        let next_start = *range.start();
        let next_end = *range.end();

        if next_start <= end + 1 {
            end = end.max(next_end);
        } else {
            count += end - start + 1;
            start = next_start;
            end = next_end;
        }
    }
    count += end - start + 1;
    count
}
