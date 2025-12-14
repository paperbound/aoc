use raoc2025::utils::*;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    mem::swap,
};

fn main() {
    let input = read_to_string("inputs/day08.txt");
    let answer1 = part1(&input);
    let answer2 = part2(&input);

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

struct Point(usize, usize, usize);

fn distance(a: &Point, b: &Point) -> usize {
    let dx = a.0.abs_diff(b.0);
    let dy = a.1.abs_diff(b.1);
    let dz = a.2.abs_diff(b.2);
    dx * dx + dy * dy + dz * dz
}

fn get_points(input: &str) -> Vec<Point> {
    input.lines().map(|line| {
        let coords: Vec<usize> = line.split(',').map(|c|
            c.parse::<usize>().unwrap()).collect();
        Point(coords[0], coords[1], coords[2])
    })
    .collect()
}

fn part1(input: &str) -> usize {
    let steps = 1000;
    let jboxes = get_points(input);
    let n = jboxes.len();
    let mut edges = BinaryHeap::with_capacity(n * (n-1) /2);
    for i in 0..n {
        for j in (i+1)..n {
            let line = distance(&jboxes[i], &jboxes[j]);
            edges.push((Reverse(line), i, j));
        }
    }

    let mut step = 0;
    let mut circuits: Vec<usize> = (0..n).collect();
    while step < steps && let Some((_, i, j)) = edges.pop() {
        step += 1;
        let mut v1 = circuits[i];
        let mut v2 = circuits[j];
        if v1 == v2 {
            continue;
        }

        if v1 > v2 { // use rank to merge
            swap(&mut v1, &mut v2);
        }

        for v in circuits.iter_mut() {
            if *v == v2 {
                *v = v1;
            }
        }
    }

    let mut counts:HashMap<usize, usize> = HashMap::new();
    for &circuit in &circuits {
        *counts.entry(circuit).or_insert(0) += 1;
    }
    let mut counts:Vec<usize> = counts.into_values().collect();
    counts.sort_unstable_by_key(|count| Reverse(*count));
    counts.iter().take(3).product::<usize>()
}

fn part2(input: &str) -> usize {
    let jboxes = get_points(input);
    let n = jboxes.len();
    let mut edges = BinaryHeap::with_capacity(n * (n-1) /2);
    for i in 0..n {
        for j in (i+1)..n {
            let line = distance(&jboxes[i], &jboxes[j]);
            edges.push((Reverse(line), i, j));
        }
    }

    let mut product = 0;
    let mut circuits: Vec<usize> = (0..n).collect();
    while let Some((_, i, j)) = edges.pop() {
        let mut v1 = circuits[i];
        let mut v2 = circuits[j];
        if v1 == v2 {
            continue;
        }

        if v1 > v2 { // use rank to merge
            swap(&mut v1, &mut v2);
        }

        let mut connections = 0;
        for v in circuits.iter_mut() {
            if *v == v2 {
                *v = v1;
                connections += 1;
            } else if *v == v1 {
                connections += 1;
            }
        }

        if connections == n {
            product = jboxes[i].0 * jboxes[j].0;
        }
    }
    product
}
