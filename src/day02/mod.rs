const RPS: &str = include_str!("input.txt");
pub(crate) fn main() {
    let score: u32 = RPS
        .lines()
        .map(|l| {
            let mut split = l.chars();
            let p1 = split.next().map(to_enum).unwrap();
            let p2 = split.nth(1).map(to_enum).unwrap();
            match p2 {
                RPSEnum::Rock => match p1 {
                    RPSEnum::Rock => 3,
                    RPSEnum::Paper => 1,
                    RPSEnum::Scissors => 2,
                },

                RPSEnum::Paper => {
                    3 + match p1 {
                        RPSEnum::Rock => 1,
                        RPSEnum::Paper => 2,
                        RPSEnum::Scissors => 3,
                    }
                }

                RPSEnum::Scissors => {
                    6 + match p1 {
                        RPSEnum::Rock => 2,
                        RPSEnum::Paper => 3,
                        RPSEnum::Scissors => 1,
                    }
                }
            }
        })
        .sum();

    println!("total: {score}")
}
fn to_enum(c: char) -> RPSEnum {
    match c {
        'A' | 'X' => RPSEnum::Rock,
        'B' | 'Y' => RPSEnum::Paper,
        'C' | 'Z' => RPSEnum::Scissors,
        _ => unreachable!(),
    }
}

enum RPSEnum {
    Rock,
    Paper,
    Scissors,
}
