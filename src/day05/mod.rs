use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");
pub(crate) fn main() {
    let mut stacks = parse_stacks(INPUT);
    let instructions: Vec<_> = INPUT
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| {
            let mut split = l.split_whitespace();
            _ = split.next();
            let count: usize = split.next().unwrap().parse().unwrap();
            _ = split.next();
            let from: u8 = split.next().unwrap().parse().unwrap();
            _ = split.next();
            let to: u8 = split.next().unwrap().parse().unwrap();
            _ = split.next();
            (count, from, to)
        })
        .collect();
    part_one(&instructions, &mut stacks.clone());
    part_two(&instructions, &mut stacks);
}

fn part_two(instructions: &[(usize, u8, u8)], stacks: &mut HashMap<u8, Vec<char>>) {
    for (count, from, to) in instructions {
        let mut temp_s = Vec::with_capacity(*count);
        for _ in 0..*count {
            let c = stacks.get_mut(from).unwrap().pop().unwrap();
            temp_s.push(c);
        }
        for c in temp_s.iter().rev() {
            stacks.entry(*to).and_modify(|s| s.push(*c));
        }
    }
    print!("part two: ");
    for i in 1..=stacks.len() {
        print!("{}", stacks[&(i as u8)].last().unwrap())
    }
    println!()
}

fn part_one(instructions: &[(usize, u8, u8)], stacks: &mut HashMap<u8, Vec<char>>) {
    for (count, from, to) in instructions {
        for _ in 0..*count {
            let c = stacks.get_mut(from).unwrap().pop().unwrap();
            stacks.entry(*to).and_modify(|s| s.push(c));
        }
    }
    print!("part one: ");
    for i in 1..=stacks.len() {
        print!("{}", stacks[&(i as u8)].last().unwrap())
    }
    println!()
}

fn parse_stacks(input: &str) -> HashMap<u8, Vec<char>> {
    let mut stack_lines: Vec<_> = input.lines().take_while(|l| !l.is_empty()).collect();
    stack_lines.reverse();
    let mut stacks = HashMap::new();
    for l in &stack_lines[1..] {
        let mut idx = 1;
        let mut char_iter = l.chars();
        while let Some(c) = char_iter.next() {
            match c {
                '[' => {
                    let krate = char_iter.next().unwrap();
                    stacks
                        .entry(idx)
                        .and_modify(|s: &mut Vec<char>| s.push(krate))
                        .or_insert_with(|| vec![krate]);
                    _ = char_iter.next();
                    _ = char_iter.next();
                }
                ' ' => {
                    for _ in 0..3 {
                        char_iter.next();
                    }
                }
                _ => unreachable!(),
            }
            idx += 1;
        }
    }
    stacks
}
