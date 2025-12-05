use raoc2025::utils::*;

fn main() {
    let input = read_to_string("inputs/day04.txt");
    let answer1 = part1(&input);

    println!("Part 1: {}", answer1);
}

fn part1(input: &str) -> u64 {
    let mut count = 0;
    let directions: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1), (0, -1),
        ( 1, -1), ( 1, 0),  (1, 1), (0,  1),
    ];

    let lines: Vec<&str> = input.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        for (j, el) in line.chars().enumerate() {
            let l = lines.len() as isize;
            let b = line.len() as isize;

            let mut ncount = 0;

            for (dx, dy) in directions {
                let x = i as isize + dx;
                let y = j as isize + dy;

                if x < 0 || x >= l || y < 0 || y >=b {
                    continue;
                }

                let nline = lines[x as usize];
                if nline.as_bytes()[y as usize] == b'@' {
                    ncount += 1;
                }
            }

            if el != '@' {
                // print!("x ");
                // if j as isize == b-1 {
                //     println!();
                // }
                continue
            // } else {
            //     print!("{} ", ncount);
            // }

            // if j as isize == b-1 {
            //     println!();
            }


            if ncount < 4 {
                count += 1;
            }
        }
    }

    count
}
