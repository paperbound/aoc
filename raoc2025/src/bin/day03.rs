use raoc2025::utils::*;

fn main() {
    let input = read_to_string("inputs/day03.txt");
    let answer1 = part1(&input);

    println!("Part 1: {}", answer1);
}

fn max_joltage(line: &str) -> u16 {
    let (mut b, mut bi) = ('0', 0);
    let (mut sb, mut sbi) = ('0', 0);

    for (i, c) in line.chars().enumerate() {
        if c > b {
            sb = b;
            sbi = bi;
            b = c;
            bi = i;
        } else if c > sb {
            sb = c;
            sbi = i;
        }
    }

    let mut s = String::new();
    if bi+1 == line.chars().count() {
        s.push(sb);
        s.push(b);
    } else if bi < sbi {
        s.push(b);
        s.push(sb);
    } else {
        let mut sbab = '0';
        for c in line[bi+1..].chars() {
            if c > sbab {
                sbab = c;
            }
        }
        s.push(b);
        s.push(sbab);
    }
    println!("{} {}", line, s);
    s.parse::<u16>().unwrap()
}

fn part1(input: &str) -> u16 {
    input.lines()
        .map(max_joltage)
        .sum()
}

