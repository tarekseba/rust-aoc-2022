use std::fs;

const UPPERCASE_PRIORITY: u8 = 27;
const LOWERCASE_PRIORITY: u8 = 1;

const CHARCODE_LOWERBOUND: u8 = 97;
const CHARCODE_UPPERBOUND: u8 = 65;

#[allow(unused)]
const SAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

fn get_common_item(part_one: &str, part_two: &str) -> Option<char> {
    let res = part_one
        .chars()
        .find(|a| part_two.contains(|element| element == *a));
    res
}

pub fn run_part_one() {
    // let x = SAMPLE
    let result = fs::read_to_string("src/day3.input")
        .unwrap()
        .lines()
        .map(|backpack: &str| -> u32 {
            let length = backpack.len();
            get_common_item(&backpack[0..length / 2], &backpack[length / 2..])
                .map(|item| match item.is_lowercase() {
                    true => LOWERCASE_PRIORITY + item as u8 - CHARCODE_LOWERBOUND,
                    false => UPPERCASE_PRIORITY + item as u8 - CHARCODE_UPPERBOUND,
                })
                .unwrap()
                .into()
        })
        .sum::<u32>();
    println!("{result}");
}

pub fn run_part_two() -> Option<u32> {
    let bags = fs::read_to_string("src/day3.input").unwrap();
    let mut bags = bags.lines();
    let mut sum = 0;
    while let Some(backpack) = bags.next() {
        sum += bags.next().and_then(|x| {
            bags.next().and_then(|y| {
                backpack
                    .chars()
                    .find(|item| {
                        x.contains(|element| element == *item)
                            && y.contains(|element| element == *item)
                    })
                    .map(|item| match item.is_lowercase() {
                        true => LOWERCASE_PRIORITY + item as u8 - CHARCODE_LOWERBOUND,
                        false => UPPERCASE_PRIORITY + item as u8 - CHARCODE_UPPERBOUND,
                    })
            })
        })? as u32
    }
    println!("{sum}");
    Some(sum)
}
