use raoc2025::utils::*;

fn main() {
    let input = read_to_string("inputs/day04.txt");
    let answer1 = part1(&input);
    let answer2 = part2(&input);

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn part2(input: &str) -> u64 {
    let mut removed = 0;
    let directions: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1), (0, -1),
        ( 1, -1), ( 1, 0),  (1, 1), (0,  1),
    ];

    let mut lines: Vec<Vec<u8>> = input.lines()
            .map(|s| s.as_bytes().to_vec())
            .collect();

    let l = lines.len() as isize;
    let b = lines[0].len() as isize;

    loop {
        let mut nlines = lines.clone();
        let mut changed = 0;

        for i in 0..l {
            for j in 0..b {
                let mut ncount = 0;

                for (dx, dy) in directions {
                    let x = i as isize + dx;
                    let y = j as isize + dy;

                    if x < 0 || x >= l || y < 0 || y >=b {
                        continue;
                    }

                    if nlines[x as usize][y as usize] == b'@' {
                        ncount += 1;
                    }
                }

                let el = nlines[i as usize][j as usize];
                if el == b'@' && ncount < 4 {
                    // print!("x ");
                    // if j as isize == b-1 {
                    //     println!();
                    // }
                    nlines[i as usize][j as usize] = b'.';
                    removed += 1;
                    changed += 1;
                // } else {
                //     print!("{} ", ncount);
                // }

                // if j as isize == b-1 {
                //     println!();
                }
            }
        }

        lines = nlines;
        if changed == 0 {
            break;
        }
    }

    removed
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
