use ndarray::{Array2, ArrayView1};
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

fn translate_sum(arr: ArrayView1<&str>) -> usize {
    let n = arr[[1]].len();
    let mut numbers: Vec<usize> = vec![0; n];
    for region in arr {
        for (i, c) in region.chars().rev().enumerate() {
            if c.is_ascii_digit() {
                numbers[i] *= 10;
                numbers[i] += c.to_digit(10).unwrap() as usize;
            }
        }
    }
    numbers.into_iter().sum()
}

fn translate_mul(arr: ArrayView1<&str>) -> usize {
    let n = arr[[1]].len();
    let mut numbers: Vec<usize> = vec![0; n];
    for region in arr {
        for (i, c) in region.chars().rev().enumerate() {
            if c.is_ascii_digit() {
                numbers[i] *= 10;
                numbers[i] += c.to_digit(10).unwrap() as usize;
            }
        }
    }
    numbers.into_iter().product()
}

fn part2(input: &str) -> usize {
    let mut sum = 0;
    let rows = input.lines().count() - 1;
    let columns = input.lines().nth(0).unwrap().split_whitespace().count();
    let mut grid: Array2<&str> = Array2::default((rows, columns));

    let mut indexes: Vec<usize> = Vec::new();
    let lines = input.lines().enumerate();
    if let Some((_, ops)) = lines.last() {
        for (index, _) in ops.match_indices("*") {
            indexes.push(index);
        }
        for (index, _) in ops.match_indices("+") {
            indexes.push(index);
        }
        indexes.push(ops.len()+3);
    }
    indexes.sort();

    let lines = input.lines().enumerate();
    for (row, line) in lines.clone().take(rows) {
        let mut column = 0;
        for pair in indexes.windows(2) {
            if let Some(numbers) = line.get(pair[0]..pair[1]-1) {
                grid[[row, column]] = numbers;
                column += 1;
            }
        }
    }

    let lines = input.lines().enumerate();
    if let Some((_, ops)) = lines.last() {
        for (column, op) in ops.split_whitespace().enumerate() {
            match op {
                "*" => sum += translate_mul(grid.column(column)),
                "+" => sum += translate_sum(grid.column(column)),
                _ => panic!("oh no!"),
            }
        }
    }
    sum
}
