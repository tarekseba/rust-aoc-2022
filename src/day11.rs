use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{self, complete::multispace1},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

#[derive(Debug)]
enum Val {
    Value(u32),
    Old,
}

impl Val {
    fn value(&self, old: u32) -> u32 {
        use Val::*;
        match self {
            Value(ref n) => n.clone(),
            Old => old,
        }
    }
}

#[derive(Debug)]
enum Op {
    Add(Val),
    Mul(Val),
}

impl Op {
    fn from(operand: &str, number: Val) -> Result<Op, String> {
        match operand {
            "*" => Ok(Op::Mul(number)),
            "+" => Ok(Op::Add(number)),
            _ => Err(String::from("Wrong operand")),
        }
    }

    fn eval(&self, lhs: u32) -> u32 {
        use Op::*;
        match self {
            Add(num) => lhs + num.value(lhs),
            Mul(num) => lhs * num.value(lhs),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    operation: Op,
    divisible_by: u32,
    if_true: u32,
    if_false: u32,
}

fn parse_items(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("Starting items: ")(input)?;
    separated_list0(tag(", "), character::complete::u32)(input)
}

fn parse_value(input: &str) -> IResult<&str, Val> {
    alt((
        tag("old").map(|_| Val::Old),
        nom::character::complete::u32.map(|number| Val::Value(number)),
    ))(input)
}

fn parse_operation(input: &str) -> IResult<&str, Op> {
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("Operation: new = old ")(input)?;
    map(
        separated_pair(alt((tag("*"), tag("+"))), tag(" "), parse_value),
        |(op, number)| Op::from(op, number).unwrap(),
    )(input)
}

fn parse_condition(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (input, _) = multispace1(input)?;
    let (input, divisible_by) =
        preceded(tag("Test: divisible by "), nom::character::complete::u32)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, if_true) = preceded(
        tag("If true: throw to monkey "),
        nom::character::complete::u32,
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, if_false) = preceded(
        tag("If false: throw to monkey "),
        nom::character::complete::u32,
    )(input)?;
    Ok((input, (divisible_by, if_true, if_false)))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, _) = nom::character::complete::u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, items) = parse_items(input)?;
    let (input, op) = parse_operation(input)?;
    let (input, (divisible_by, if_true, if_false)) = parse_condition(input)?;
    Ok((
        input,
        Monkey {
            items,
            operation: op,
            divisible_by,
            if_true,
            if_false,
        },
    ))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    let separator = tag("\n\n");
    separated_list1(separator, parse_monkey)(input)
}

#[allow(unused)]
const SAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

pub fn run_part_one() -> Result<(), String> {
    // let input = SAMPLE;
    let input = &fs::read_to_string("src/day11.input").unwrap();

    let mut monkeys = parse_monkeys(input).map_err(|err| err.to_string())?.1;

    let mut h_map: HashMap<usize, usize> = HashMap::new();

    for _ in 0..20 {
        for index in 0..monkeys.len() {
            let item_count = monkeys.get(index).unwrap().items.len();
            h_map
                .entry(index)
                .and_modify(|x| *x = *x + item_count)
                .or_insert(item_count);

            while monkeys.get(index).unwrap().items.len() > 0 {
                let monkey = monkeys.get_mut(index).unwrap();

                let mut item = monkey.items.pop().unwrap();
                item = monkey.operation.eval(item) / 3;

                let divisible = item % monkey.divisible_by == 0;

                let if_true = monkey.if_true;

                let if_false = monkey.if_false;

                if divisible {
                    monkeys.get_mut(if_true as usize).unwrap().items.push(item);
                } else {
                    monkeys.get_mut(if_false as usize).unwrap().items.push(item)
                }
            }
        }
    }
    let mut vec = h_map.into_iter().map(|(_, y)| y).collect::<Vec<usize>>();
    vec.sort_by(|item1, item2| item2.cmp(item1));
    println!("{:?}", vec.iter().take(2).product::<usize>());
    Ok(())
}
