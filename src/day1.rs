use std::fs;

pub fn run_part_1() -> Result<i32, String> {
    let mut scores: Vec<i32> = vec![];
    let mut index: usize = 0;
    let input = fs::read_to_string("src/day1.input").map_err(|err| err.to_string())?;
    for line in input.lines() {
        if line.is_empty() {
            index += 1;
        } else {
            let number = line.parse::<i32>().map_err(|err| err.to_string())?;
            match scores.get_mut(index) {
                None => scores.push(number),
                Some(value) => *value += number,
            }
        }
    }
    scores
        .into_iter()
        .max()
        .ok_or(String::from("Error getting max"))
}

pub fn run_part_2() -> Result<i32, String> {
    let mut scores: Vec<i32> = vec![];
    let mut index: usize = 0;
    let input = fs::read_to_string("src/day1.input").map_err(|err| err.to_string())?;
    for line in input.lines() {
        if line.is_empty() {
            index += 1;
        } else {
            let number = line.parse::<i32>().map_err(|err| err.to_string())?;
            match scores.get_mut(index) {
                None => scores.push(number),
                Some(value) => *value += number,
            }
        }
    }
    scores.sort_by(|a, b| b.cmp(a));
    Ok(scores.into_iter().take(3).sum())
}
