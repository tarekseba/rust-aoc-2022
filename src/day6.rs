use std::fs;

pub fn run_part_one() {
    let input = fs::read_to_string("src/day6.input").unwrap();
    let mut buf = "".to_string();
    let count = input
        .chars()
        .take_while(|character| {
            if buf.contains(*character) {
                let character_index = buf.find(*character);
                match character_index {
                    Some(index) => buf = format!("{}{}", &buf[index + 1..], *character),
                    None => (),
                }
            } else {
                buf.push(*character);
                if buf.len() >= 4 {
                    return false;
                }
            };
            true
        })
        .count();
    println!("{}", count + 1)
}
