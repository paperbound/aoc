use raoc2025::utils::*;
use std::ops::RangeInclusive;

fn main() {
    let input = read_to_string("inputs/day02.txt");
    let answer1 = part1(&input);
    let answer2 = part2(&input);

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn invalid_id_v1(num: &u64) -> bool {
    let num_s = num.to_string();
    let middle = num_s.len() / 2;

    num_s[0..middle] == num_s[middle..]
}

fn invalid_id_v2(num: &u64) -> bool {
    let num_s = num.to_string();
    let len = num_s.len();

    if len <= 1 {
        return false;
    }

    'find: for piece in 1..len / 2 + 1 {
        if len % piece != 0 {
            continue;
        }

        for part in 1..len / piece {
            if num_s[0..piece] != num_s[part * piece..(part + 1) * piece] {
                continue 'find;
            }
        }

        return true;
    }

    false
}

fn id_range(range: &str) -> RangeInclusive<u64> {
    let (begin, end) = range.split_once('-').unwrap();
    let b = begin.parse::<u64>().unwrap();
    let e = end.parse::<u64>().unwrap();

    b..=e
}

fn part1(input: &str) -> u64 {
    input
        .split(',')
        .map(id_range)
        .map(|r| r.filter(invalid_id_v1).sum::<u64>())
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .split(',')
        .map(id_range)
        .map(|r| r.filter(invalid_id_v2).sum::<u64>())
        .sum()
}
