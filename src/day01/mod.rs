const FOODS: &str = include_str!("input.txt");

pub(crate) fn main() {
    let mut cals = Vec::new();
    let mut tmp = 0;
    for line in FOODS.lines() {
        if let Ok(cal) = line.parse::<u32>() {
            tmp += cal;
        } else {
            cals.push(tmp);
            tmp = 0;
        }
    }

    cals.sort();
    let max_3: u32 = cals.iter().rev().take(3).sum();

    println!("3 max calories: {max_3}");
}
