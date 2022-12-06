use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

pub(crate) fn main() {
    let v: Vec<_> = INPUT.lines().collect();

    let result: u32 = v
        .chunks_exact(3)
        .map(|l| {
            let l1: HashSet<char> = HashSet::from_iter(l[0].chars());
            let l2 = HashSet::from_iter(l[1].chars());
            let l3 = HashSet::from_iter(l[2].chars());
            let same = *l1
                .intersection(&l2)
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&l3)
                .next()
                .unwrap();
            priority(same) as u32
        })
        .sum();
    println!("result: {result}");
    let t = priority('A');
    println!("test: {t}");
}

fn priority(c: char) -> u8 {
    if c.is_uppercase() {
        ((c as u8) ^ b' ') - b'a' + 27
    } else {
        ((c as u8) ^ b' ') - b'A' + 1
    }
}
