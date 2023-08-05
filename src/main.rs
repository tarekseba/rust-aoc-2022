mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    println!("day 1: {:?}", day1::run_part_2());
    println!("day 2: {:?}", day2::run_part_one());
    println!("day 2: {:?}", day2::run_part_two());
    println!("----------DAY-4------------");
    day4::run_part_one();
    day4::run_part_two();
    println!("----------DAY-5------------");
    day5::run_part_one();
    day5::run_part_two();
    println!("----------DAY-6------------");
    day6::run_part_one();
    day6::run_part_two();
    println!("----------DAY-7------------");
    day7::run_part_one();
    day7::run_part_two();
    println!("----------DAY-8------------");
    day8::run_both_parts();
    day9::run_part_one();
    day10::run_part_one();
}
