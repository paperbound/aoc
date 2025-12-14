use raoc2025::utils::*;
use std::{
    collections::BinaryHeap,
    cmp::{min, max},
};

fn main() {
    let input = read_to_string("inputs/day09.txt");
    let answer1 = part1(&input);
    let answer2 = part2(&input);

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

struct Point(usize, usize);

fn calculate_area(a: &Point, b: &Point) -> usize {
    let dx = a.0.abs_diff(b.0);
    let dy = a.1.abs_diff(b.1);
    (dx+1) * (dy+1)
}

fn get_points(input: &str) -> Vec<Point> {
    input.lines().map(|line| {
        let coords: Vec<usize> = line.split(',').map(|c|
            c.parse::<usize>().unwrap()).collect();
        Point(coords[0], coords[1])
    })
    .collect()
}

fn part1(input: &str) -> usize {
    let mut largest_area = 0;
    let tiles = get_points(input);
    let n = tiles.len();
    let mut edges = BinaryHeap::with_capacity(n * (n-1) /2);
    for i in 0..n {
        for j in (i+1)..n {
            let area = calculate_area(&tiles[i], &tiles[j]);
            edges.push((area, i, j));
        }
    }

    if let Some((area, _, _)) = edges.pop() {
        largest_area = area;
    }
    largest_area
}

fn part2(input: &str) -> usize {
    let mut largest_area = 0;
    let tiles = get_points(input);
    let n = tiles.len();
    let mut edges = BinaryHeap::with_capacity(n * (n-1) /2);
    for i in 0..n {
        for j in (i+1)..n {
            let area = calculate_area(&tiles[i], &tiles[j]);
            edges.push((area, i, j));
        }
    }

    while let Some((area, i, j)) = edges.pop() {
        let mut inside = true;
        let mut a = tiles.last().unwrap();
        let x_min = min(tiles[i].0, tiles[j].0);
        let x_max = max(tiles[i].0, tiles[j].0);
        let y_min = min(tiles[i].1, tiles[j].1);
        let y_max = max(tiles[i].1, tiles[j].1);
        for b in tiles.iter() {
            let cx_min = min(a.0, b.0);
            let cx_max = max(a.0, b.0);
            let cy_min = min(a.1, b.1);
            let cy_max = max(a.1, b.1);
            // does the rectangle protrude
            let vertical_bound = (a.0 == b.0) &&
                (a.0 > x_min && a.0 < x_max && cy_min < y_max && cy_max > y_min);
            let horizontal_bound = (a.0 != b.0) &&
                (a.1 > y_min && a.1 < y_max && cx_min < x_max && cx_max > x_min);
            if vertical_bound || horizontal_bound {
                inside = false;
                break;
            }
            a = b;
        }
        if inside {
            largest_area = area;
            break;
        }
    }
    largest_area
}
