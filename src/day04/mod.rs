const INPUT: &str = include_str!("input.txt");
pub(crate) fn main() {
    let partial_overlap: Vec<_> = INPUT.lines().filter(partial_overlap).collect();
    println!(
        "assignment pairs (partial overlap): {}",
        partial_overlap.len()
    );

    let full_overlap_count = partial_overlap.iter().filter(full_overlap).count();
    println!("assignment pairs (full overlap): {full_overlap_count}");
}

fn parse_assignment_pairs(line: &str) -> ((u16, u16), (u16, u16)) {
    let (elf1, elf2) = line.split_once(',').unwrap();
    let (min1, max1) = elf1.split_once('-').unwrap();
    let (min2, max2) = elf2.split_once('-').unwrap();

    let min1: u16 = min1.parse().unwrap();
    let max1: u16 = max1.parse().unwrap();
    let min2: u16 = min2.parse().unwrap();
    let max2: u16 = max2.parse().unwrap();
    ((min1, max1), (min2, max2))
}

fn partial_overlap(line: &&str) -> bool {
    let ((min1, max1), (min2, max2)) = parse_assignment_pairs(line);
    (min1 <= min2 && max1 >= min2) || (min2 <= min1 && max2 >= min1)
}

fn full_overlap(line: &&&str) -> bool {
    let ((min1, max1), (min2, max2)) = parse_assignment_pairs(line);

    (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1)
}
