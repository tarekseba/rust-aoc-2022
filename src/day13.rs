use std::{cmp::Ordering, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::{map, opt},
    multi::many0,
    sequence::{preceded, separated_pair},
    IResult,
};

#[allow(unused)]
const SAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

#[derive(Debug, Clone)]
enum List {
    Value(u32),
    Cons(Vec<List>),
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        match self {
            List::Value(ref lhs) => match other {
                List::Value(ref rhs) => lhs == rhs,
                List::Cons(_) => {
                    let transformed_lhs = List::Cons(vec![List::Value(lhs.clone())]);
                    transformed_lhs == *other
                }
            },
            List::Cons(ref lhs) => match other {
                List::Value(ref rhs) => {
                    let transformed_rhs = List::Cons(vec![List::Value(rhs.clone())]);
                    *self == transformed_rhs
                }
                List::Cons(ref rhs) => {
                    lhs.iter().zip(rhs.iter()).all(|(l, r)| l == r) && lhs.len() == rhs.len()
                }
            },
        }
    }
}

impl Eq for List {}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            List::Value(ref lhs) => match other {
                List::Value(ref rhs) => lhs.cmp(rhs),
                List::Cons(_) => {
                    let transformed_lhs = List::Cons(vec![List::Value(lhs.clone())]);
                    transformed_lhs.cmp(other)
                }
            },
            List::Cons(ref lhs) => match other {
                List::Value(ref rhs) => {
                    let transformed_rhs = List::Cons(vec![List::Value(rhs.clone())]);
                    self.cmp(&transformed_rhs)
                }
                List::Cons(ref rhs) => lhs.cmp(&rhs),
            },
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_values(input: &str) -> IResult<&str, Vec<List>> {
    // println!("inside parse_values => input : {input}");
    let (input, val) = many0(preceded(
        opt(tag(",")),
        alt((map(complete::u32, |i| List::Value(i)), parse_list)),
    ))(input)?;
    Ok((input, val))
}

fn parse_list(input: &str) -> IResult<&str, List> {
    // println!("inside parse_list => input   : {input}");
    let (input, _) = tag("[")(input)?;
    let (input, values) = parse_values(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, List::Cons(values)))
}

pub fn run_part_one() -> Result<(), String> {
    // let input = SAMPLE;
    let input = fs::read_to_string("src/day13.input").unwrap();
    let splitted_input = input.split("\n\n").collect::<Vec<&str>>();

    let entries = splitted_input
        .iter()
        .map(|pair| {
            separated_pair(parse_list, tag("\n"), parse_list)(pair)
                .map_err(|e| e.to_string())
                .map(|e| e.1)
        })
        .collect::<Result<Vec<(List, List)>, String>>()?;

    let results = entries
        .iter()
        .enumerate()
        .map(|(index, pair)| (index, pair.0.cmp(&pair.1)))
        .filter_map(|(index, cmp)| {
            if cmp != Ordering::Greater {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("--------------------------------- DAY 13 -----------------------------------");
    println!("{:?}", results);
    println!("--------------------------------- PART 2 -----------------------------------");

    Ok(())
}

pub fn run_part_two() -> Result<(), String> {
    // let input = SAMPLE;
    let input = fs::read_to_string("src/day13.input").unwrap();
    let splitted_input = input.split("\n\n").collect::<Vec<&str>>();

    let entries = splitted_input
        .iter()
        .map(|pair| {
            separated_pair(parse_list, tag("\n"), parse_list)(pair)
                .map_err(|e| e.to_string())
                .map(|e| vec![e.1 .0, e.1 .1])
        })
        .collect::<Result<Vec<Vec<List>>, String>>()?;

    let mut entries = entries.iter().flatten().collect::<Vec<&List>>();

    let (el1, el2) = (
        List::Cons(vec![List::Cons(vec![List::Value(2)])]),
        List::Cons(vec![List::Cons(vec![List::Value(6)])]),
    );

    entries.push(&el1);
    entries.push(&el2);
    entries.sort();

    let product: usize = entries
        .iter()
        .enumerate()
        .fold(vec![], |mut acc, (index, e)| {
            if **e == el1 || **e == el2 {
                acc.push(index + 1)
            }
            acc
        })
        .iter()
        .product();
    println!("{:?}", product);
    println!("{}", "-".repeat(75));
    Ok(())
}
