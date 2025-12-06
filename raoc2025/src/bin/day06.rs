use ndarray::Array2;
use raoc2025::utils::*;

fn main() {
    let input = read_to_string("inputs/day06.txt");
    let answer1 = part1(&input);
    let answer2 = part2(&input);

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn part1(input: &str) -> usize {
    let mut sum = 0;
    let rows = input.lines().count() - 1;
    let columns = input.lines().nth(0).unwrap().split_whitespace().count();
    let mut math: Array2<usize> = Array2::zeros((rows, columns));

    let lines = input.lines().enumerate();
    for (row, line) in lines.take(rows) {
        for (column, number) in line.split_whitespace().enumerate() {
            math[[row, column]] = number.parse::<usize>().unwrap();
        }
    }

    let lines = input.lines().enumerate();
    if let Some((_, ops)) = lines.last() {
        for (column, op) in ops.split_whitespace().enumerate() {
            match op {
                "*" => sum += math.column(column).product(),
                "+" => sum += math.column(column).sum(),
                _ => panic!("oh no!"),
            }
        }
    }
    sum
}

fn part2(input: &str) -> usize {
    input.lines().count()
}
