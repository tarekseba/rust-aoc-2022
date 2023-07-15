static SAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

pub fn run() -> Result<i32, String> {
    let mut scores: Vec<i32> = vec![];
    let mut index: usize = 0;
    for line in SAMPLE.lines() {
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
