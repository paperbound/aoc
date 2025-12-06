use raoc2025::utils::*;

fn main() {
    let input = read_to_string("inputs/day03.txt");
    let answer1 = part1(&input);
    let answer2 = part2(&input);

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn largest(bank: &[u8]) -> (usize, u8) {
    let (mut l, mut li) = (0, 0);

    for (i, b) in bank.iter().enumerate() {
        if *b > l {
            l = *b;
            li = i;
        }
    }

    (li, l)
}

fn max_joltage_2(line: &str) -> u64 {
    let bs = line.as_bytes();
    let bl = line.len();

    let mut max: u64 = 0;

    let (i, b) = largest(&bs[..bl - 1]);
    max += ((b - b'0') as u64) * 10;

    let (_, b) = largest(&bs[i + 1..bl]);
    max += (b - b'0') as u64;

    max
}

fn max_joltage_12(line: &str) -> u64 {
    let bs = line.as_bytes();
    let bl = line.len();

    let mut max = 0;
    let mut start = 0;
    let mut power = 100_000_000_000;

    for i in 0..12 {
        let lim = bl + i - 11;
        let (i, big) = largest(&bs[start..lim]);

        max += ((big - b'0') as u64) * power;
        power /= 10;
        start += i + 1;
    }

    max
}

fn part1(input: &str) -> u64 {
    input.lines().map(max_joltage_2).sum()
}

fn part2(input: &str) -> u64 {
    input.lines().map(max_joltage_12).sum()
}
