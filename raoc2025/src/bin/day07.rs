use raoc2025::utils::*;

fn main() {
    let input = read_to_string("inputs/day07.txt");
    let answer1 = part1(&input);
    let answer2 = part2(&input);

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

enum Split {
    Beam,
    Universe,
}

fn simulate(input:&str, action: Split) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();
    let mut beams = vec![0_usize; width];
    let mut min = width / 2;
    let mut max = min;
    let mut splits = 0_usize;
    for line in lines {
        for i in min..=max {
            let c = line.as_bytes()[i];
            match c {
                b'S' => beams[i] = 1,
                b'^' if beams[i] > 0 => {
                    splits += 1;
                    let current = beams[i];
                    if i >= 1 {
                        beams[i-1] += current;
                        min = min.min(i-1);
                    }
                    if i + 1 < width {
                        beams[i+1] += current;
                        max = max.max(i+1);
                    }
                    beams[i] = 0;
                }
                _ => {}
            }
        }
    }
    match action {
        Split::Beam     => splits,
        Split::Universe => beams.into_iter().sum(),
    }
}

fn part1(input: &str) -> usize {
    simulate(input, Split::Beam)
}

fn part2(input: &str) -> usize {
    simulate(input, Split::Universe)
}
