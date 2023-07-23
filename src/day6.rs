use std::fs;

pub fn run_part_one() {
    let count = fs::read_to_string("src/day6.input")
        .unwrap()
        .chars()
        .take_while(check_marker(4))
        .count();
    println!("{}", count + 1)
}

pub fn run_part_two() {
    let count = fs::read_to_string("src/day6.input")
        .unwrap()
        .chars()
        .take_while(check_marker(14))
        .count();
    println!("{}", count + 1)
}

fn check_marker(marker_size: usize) -> Box<dyn FnMut(&char) -> bool> {
    let mut buf = String::from("");
    Box::new(move |character: &char| {
        if buf.contains(*character) {
            let character_index = buf.find(*character);
            match character_index {
                Some(index) => buf = format!("{}{}", &buf[index + 1..], *character),
                None => (),
            }
        } else {
            buf.push(*character);
            if buf.len() >= marker_size {
                return false;
            }
        };
        true
    })
}
