use std::{fs, ops::Deref};

const SAMPLE: &str = "A Y
B X
C Z";

pub fn run_part_one() -> Result<i32, String> {
    // let result = SAMPLE
    let result = fs::read_to_string("src/day2.input")
        .map_err(|err| err.to_string())?
        .lines()
        .map(|value| value.trim().split(" ").collect::<Vec<&str>>())
        .map(|vec| (RockPaper::from(vec[0]), RockPaper::from(vec[1])))
        .fold(0, |acc, (hand1, hand2)| acc + (hand1 + hand2));
    Ok(result)
}

pub fn run_part_two() -> Result<i32, String> {
    // let result = SAMPLE
    let result = fs::read_to_string("src/day2.input")
        .map_err(|err| err.to_string())?
        .lines()
        .map(|value| value.trim().split(" ").collect::<Vec<&str>>())
        .map(|vec| {
            let hand_one = RockPaper::from(vec[0]);
            let preferred_outcome = Outcome::from(vec[1]).chosen_outcome(&hand_one);
            (hand_one, preferred_outcome)
        })
        .fold(0, |acc, (hand1, hand2)| acc + (hand1 + hand2));
    Ok(result)
}

#[derive(Debug, Clone)]
enum RockPaper {
    Rock = 1,
    Paper = 2,
    Scisors = 3,
}

#[derive(PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn chosen_outcome(self, hand: &RockPaper) -> RockPaper {
        match (self, hand) {
            (Outcome::Win, RockPaper::Rock) => RockPaper::Paper,
            (Outcome::Win, RockPaper::Paper) => RockPaper::Scisors,
            (Outcome::Win, RockPaper::Scisors) => RockPaper::Rock,
            (Outcome::Lose, RockPaper::Rock) => RockPaper::Scisors,
            (Outcome::Lose, RockPaper::Paper) => RockPaper::Rock,
            (Outcome::Lose, RockPaper::Scisors) => RockPaper::Paper,
            (Outcome::Draw, x) => x.clone(),
        }
    }
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

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => Self::Win,
        }
    }
}

impl std::ops::Add<RockPaper> for RockPaper {
    type Output = i32;

    fn add(self, rhs: RockPaper) -> Self::Output {
        (match (self, &rhs) {
            (RockPaper::Rock, RockPaper::Paper) => 6,
            (RockPaper::Rock, RockPaper::Scisors) => 0,
            (RockPaper::Paper, RockPaper::Rock) => 0,
            (RockPaper::Paper, RockPaper::Scisors) => 6,
            (RockPaper::Scisors, RockPaper::Rock) => 6,
            (RockPaper::Scisors, RockPaper::Paper) => 0,
            _ => 3,
        }) + rhs as i32
    }
}
