use std::fs;

const SAMPLE: &str = "A Y
B X
C Z";

pub fn run() -> Result<i32, String> {
    // let result = SAMPLE
    let result = fs::read_to_string("src/day2.input")
        .map_err(|err| err.to_string())?
        .lines()
        .map(|value| value.trim().split(" ").collect::<Vec<&str>>())
        .map(|vec| (RockPaper::from(vec[0]), RockPaper::from(vec[1])))
        .fold(0, |acc, (hand1, hand2)| acc + (hand1 + hand2));
    Ok(result)
}

#[derive(Debug)]
enum RockPaper {
    Rock,
    Paper,
    Scisors,
}

impl From<&str> for RockPaper {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => RockPaper::Rock,
            "B" | "Y" => RockPaper::Paper,
            "C" | "Z" => RockPaper::Scisors,
            _ => RockPaper::Rock,
        }
    }
}

impl std::ops::Add<RockPaper> for RockPaper {
    type Output = i32;

    fn add(self, rhs: RockPaper) -> Self::Output {
        match (self, rhs) {
            (RockPaper::Rock, RockPaper::Rock) => 1 + 3,
            (RockPaper::Rock, RockPaper::Paper) => 2 + 6,
            (RockPaper::Rock, RockPaper::Scisors) => 3,
            (RockPaper::Paper, RockPaper::Rock) => 1,
            (RockPaper::Paper, RockPaper::Paper) => 2 + 3,
            (RockPaper::Paper, RockPaper::Scisors) => 3 + 6,
            (RockPaper::Scisors, RockPaper::Rock) => 1 + 6,
            (RockPaper::Scisors, RockPaper::Paper) => 2,
            (RockPaper::Scisors, RockPaper::Scisors) => 3 + 3,
        }
    }
}
