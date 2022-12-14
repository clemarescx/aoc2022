use std::{collections::HashSet, fmt::Display, vec};

const INPUT: &str = include_str!("input.txt");

pub(crate) fn main() {
    let commands: Vec<_> = INPUT
        .lines()
        .map(|l| {
            let (cmd, steps) = l.split_once(' ').unwrap();
            let steps: u8 = steps.parse().unwrap();
            match cmd {
                "U" => Cmd::U(steps),
                "R" => Cmd::R(steps),
                "D" => Cmd::D(steps),
                "L" => Cmd::L(steps),
                _ => unreachable!(),
            }
        })
        .collect();
    part_one(&commands);
    part_two(&commands);
}

fn part_one(commands: &[Cmd]) {
    let mut trace = HashSet::new();
    let mut tail_pos = (0, 0);
    let mut head_pos = (0, 0);
    for command in commands {
        let (head_vel, steps) = match command {
            Cmd::U(steps) => ((0, 1), steps),
            Cmd::L(steps) => ((1, 0), steps),
            Cmd::D(steps) => ((0, -1), steps),
            Cmd::R(steps) => ((-1, 0), steps),
        };

        for _ in 0..*steps {
            let new_head_pos = add(head_pos, head_vel);
            if !adjacent(new_head_pos, tail_pos) {
                tail_pos = head_pos;
            }

            head_pos = new_head_pos;
            trace.insert(tail_pos);
        }
    }
    println!("recorded tail positions: {}", trace.len());
}

fn part_two(commands: &[Cmd]) {
    let mut trace = HashSet::new();
    let start = (0, 0);
    let mut knots = vec![start; 10];
    for command in commands {
        let (head_vel, steps) = match command {
            Cmd::U(steps) => ((0, 1), steps),
            Cmd::L(steps) => ((-1, 0), steps),
            Cmd::D(steps) => ((0, -1), steps),
            Cmd::R(steps) => ((1, 0), steps),
        };

        for _ in 0..*steps {
            let mut suggested_pos = Default::default();
            knots = knots.iter().fold(vec![], |mut acc, curr| {
                if let Some(&head) = acc.last() {
                    let mut curr = *curr;
                    if !adjacent(head, curr) {
                        let dist = sub(head, curr);
                        curr = if dist.0.abs() < dist.1.abs() {
                            add(curr, (dist.0, dist.1 - dist.1.signum()))
                        } else if dist.0.abs() > dist.1.abs() {
                            add(curr, (dist.0 - dist.0.signum(), dist.1))
                        } else {
                            add(curr, (dist.0 - dist.0.signum(), dist.1 - dist.1.signum()))
                        };
                    }

                    acc.push(curr);
                } else {
                    let moved = add(*curr, head_vel);
                    suggested_pos = sub(moved, head_vel);
                    acc.push(moved);
                }
                acc
            });

            trace.insert(knots[knots.len() - 1]);
        }
    }
    println!("recorded tail positions: {}", trace.len());
}

fn adjacent((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> bool {
    x1.abs_diff(x2) <= 1 && y1.abs_diff(y2) <= 1
}

fn add((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> (i32, i32) {
    (x1 + x2, y1 + y2)
}

fn sub((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> (i32, i32) {
    (x1 - x2, y1 - y2)
}

enum Cmd {
    U(u8),
    L(u8),
    D(u8),
    R(u8),
}
impl Display for Cmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cmd::U(s) => write!(f, "UP {s}"),
            Cmd::L(s) => write!(f, "LEFT {s}"),
            Cmd::D(s) => write!(f, "DOWN {s}"),
            Cmd::R(s) => write!(f, "RIGHT {s}"),
        }
    }
}
