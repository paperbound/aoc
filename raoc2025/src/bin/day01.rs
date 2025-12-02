use raoc2025::utils::*;

fn main() {
    let input = read_to_string("inputs/day01.txt");
    let answer1 = part1(&input);
    let answer2 = part2(&input);

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
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

        dial = dial.rem_euclid(100);

        if dial == 0 {
            count += 1;
        }
    }
    count
}

fn part2(input: &str) -> i32 {
    let mut count = 0;
    let mut dial:i32= 50;
    for line in input.lines() {
        let dir = &line[0..1];
        let num: i32 = line[1..].parse().unwrap();
        let mut wound = 0;

        match dir {
            "L" => {
                // Number of times you reach 0
                if dial == 0 {
                    wound = -1;
                }
                dial -= num;
                if dial.rem_euclid(100) == 0 {
                    wound += 1;
                }
            }
            "R" => dial += num,
            _   => panic!("unknown direction {dir}!"),
        };

        let wraps = dial.div_euclid(100).abs() + wound;
        dial = dial.rem_euclid(100);
        count += wraps;
    }

    count
}

