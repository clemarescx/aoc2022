use std::{
    borrow::BorrowMut,
    collections::{HashSet, VecDeque},
};

const INPUT: &str = include_str!("input.txt");
pub(crate) fn main() {
    println!("part one:");
    part_one(INPUT);
    println!("part two:");
    part_two(INPUT);
}

fn part_one(input: &str) {
    find_marker(input, 4);
}
fn part_two(input: &str) {
    find_marker(input, 14);
}

fn find_marker(input: &str, marker_length: usize) {
    let mut h = HashSet::with_capacity(marker_length);
    let mut char_iter = input.char_indices();
    let mut queue = VecDeque::from_iter(char_iter.borrow_mut().take(marker_length).map(|(_, c)| c));

    for (i, c) in char_iter {
        h.clear();
        for c in queue.clone() {
            h.insert(c);
        }
        if h.len() == marker_length {
            println!("marker: {i}");
            return;
        }
        queue.pop_front();
        queue.push_back(c);
    }
}
