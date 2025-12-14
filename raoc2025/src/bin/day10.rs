use raoc2025::utils::*;
use std::collections::{HashSet, VecDeque};

fn main() {
    let input = read_to_string("inputs/day10.txt");
    let answer1 = part1(&input);
    let answer2 = part2(&input);

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

#[derive(Default,Debug)]
struct Machine {
    indication: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>
}

impl Machine {
    fn from_str(line: &str) -> Self {
        let mut machine = Machine::default();
        for words in line.split_whitespace() {
            if let Some(start) = words.chars().nth(0) {
                let word = &words[1..words.len()-1];
                match start {
                    '[' => { machine.indication = word.chars().map(|c| c == '#').collect() ;},
                    '(' => { machine.buttons.push(word.split(',').map(|s|
                        s.parse().unwrap()).collect());},
                    '{' => { machine.joltages = word.split(',').map(|s|
                        s.parse().unwrap()).collect();}
                    _ => todo!()
                }
            }
        }
        machine
    }

    fn start(&self) -> usize {
        let mut best = usize::MAX;
        let mut queue = VecDeque::new();
        let start = vec![false; self.indication.len()];

        queue.push_back((start.clone(), 0));
        let mut visited = HashSet::new();
        visited.insert(start.clone());

        while let Some((state, presses)) = queue.pop_front() {
            if state == self.indication {
                best = best.min(presses);
                continue;
            }

            if presses >= best {
                continue;
            }

            for button in self.buttons.iter() { // bfs
                let mut next_state = state.clone();
                for &i in button {
                    next_state[i] = !next_state[i];
                }

                if visited.insert(next_state.clone()) {
                    queue.push_back((next_state, presses + 1));
                }
            }
        }

        best
    }

    fn jolt(&self) -> usize {
        let mut best = usize::MAX;
        let mut queue = VecDeque::new();
        let start = vec![0_usize; self.indication.len()];

        queue.push_back((start.clone(), 0));
        let mut visited = HashSet::new();
        visited.insert(start.clone());

        while let Some((state, presses)) = queue.pop_front() {
            if state == self.joltages {
                best = best.min(presses);
                continue;
            }

            if presses >= best {
                continue;
            }

            for button in self.buttons.iter() { // bfs
                let mut next_state = state.clone();
                for &i in button {
                    next_state[i] += 1;
                }

                if visited.insert(next_state.clone()) {
                    queue.push_back((next_state, presses + 1));
                }
            }
        }

        best
    }
}

fn part1(input: &str) -> usize {
    input.lines().map(|line| Machine::from_str(line).start()).sum()
}

fn part2(input: &str) -> usize {
    input.lines().map(|line| Machine::from_str(line).jolt()).sum()
}
