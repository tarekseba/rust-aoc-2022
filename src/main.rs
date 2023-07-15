mod day1;
fn main() {
    match day1::run() {
        Ok(cals) => println!("{cals}"),
        Err(err) => println!("{err}"),
    }
}
