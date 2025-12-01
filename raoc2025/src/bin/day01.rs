use raoc2025::utils::*;

fn main() {
    let input = read_to_string("inputs/day01.txt");
    let answer1 = part1(&input);

    println!("Part 1: {}", answer1);
}

fn part1(input: &str) -> u16 {
    let mut count = 0;
    let mut dial:i32= 50;
    for line in input.lines() {
        let dir = &line[0..1];
        let num: i32 = line[1..].parse().unwrap();

        match dir {
            "L" => dial -= num,
            "R" => dial += num,
            _ => panic!("oh no!"),
        }

        dial = positive_modulo(dial, 100);

        if dial == 0 {
            count += 1;
        }
    }
    count
}


