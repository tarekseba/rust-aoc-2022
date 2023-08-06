use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::newline,
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

#[allow(unused)]
const SAMPLE: &str = "noop
addx 3
addx 4
noop
addx -5";

#[derive(Debug)]
enum Instruction {
    Add(i32),
    Noop,
}

impl Instruction {
    fn value(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Add(_) => 2,
        }
    }
}

impl std::ops::Add<i32> for &Instruction {
    type Output = i32;

    fn add(self, rhs: i32) -> Self::Output {
        match self {
            Instruction::Add(ref number) => number + rhs,
            Instruction::Noop => rhs,
        }
    }
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, res) = alt((tag("noop"), tag("addx")))(input)?;
    match res {
        "noop" => Ok((input, Instruction::Noop)),
        "addx" => {
            let (input, _) = tag(" ")(input)?;
            let (input, number) =
                map_res(is_a("1234567890-"), |num: &str| num.parse::<i32>())(input)?;
            Ok((input, Instruction::Add(number)))
        }
        _ => panic!("Faulty input"),
    }
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, parse_instruction)(input)
}

pub fn run_part_one() -> Result<(), String> {
    let mut cycle: usize = 0;
    let mut x_map: HashMap<usize, i32> = HashMap::new();
    let mut register: i32 = 1;
    let mut cycle_multiplier = 0;

    // let input = SAMPLE;
    let input = fs::read_to_string("src/day10.input").unwrap();
    let instructions = parse_instructions(&input).map_err(|err| err.to_string())?.1;
    instructions.iter().for_each(|inst: &Instruction| {
        let needed_cycle = 20 + 40 * cycle_multiplier;
        cycle += inst.value();
        if cycle == needed_cycle {
            let _ = x_map.insert(needed_cycle, register * needed_cycle as i32);
            register = inst + register;
            cycle_multiplier += 1;
        } else if cycle > needed_cycle
            && cycle
                .checked_sub(inst.value())
                .is_some_and(|v| v < needed_cycle)
        {
            let _ = x_map.insert(needed_cycle, register * needed_cycle as i32);
            register = inst + register;
            cycle_multiplier += 1;
        } else {
            register = inst + register;
        }
    });
    println!("------------------------DAY 10------------------------------");
    println!("Sum : {:?}", x_map.iter().map(|x| x.1).sum::<i32>());
    println!("------------------------Part 2-------------------------------");
    Ok(())
}

fn draw(screen: &mut Vec<Vec<char>>, mut cycle: usize, register: i32, instruction: &Instruction) {
    let reg: i32 = register;
    screen.get_mut(cycle / 40).unwrap();
    match instruction {
        Instruction::Add(_) => {
            if (reg - 1..=reg + 1).contains(&(cycle as i32 % 40)) {
                let character = screen
                    .get_mut(cycle / 40)
                    .expect(&((cycle / 40).to_string() + "Panicked first"))
                    .get_mut(cycle % 40)
                    .expect(&((cycle % 40).to_string() + "Panicked because out of bounds"));
                *character = '#'
            }
            cycle += 1;
            if (reg - 1..=reg + 1).contains(&(cycle as i32 % 40)) {
                let character = screen
                    .get_mut(cycle / 40)
                    .expect(&((cycle / 40).to_string() + "Panicked second"))
                    .get_mut(cycle % 40)
                    .expect("Panicked because out of bounds");
                *character = '#'
            }
        }
        Instruction::Noop => {
            let character = screen
                .get_mut(cycle / 40)
                .expect(&((cycle / 40).to_string() + "Panicked second"))
                .get_mut(cycle % 40)
                .expect("Panicked because out of bounds");
            *character = '#'
        }
    }
}

pub fn run_part_two() -> Result<(), String> {
    let mut cycle: usize = 0;
    let mut register: i32 = 1;
    let mut x: Vec<Vec<char>> = vec![
        vec!['.'; 40],
        vec!['.'; 40],
        vec!['.'; 40],
        vec!['.'; 40],
        vec!['.'; 40],
        vec!['.'; 40],
    ];
    x.iter_mut().for_each(|vec| vec.fill('.'));

    let input = fs::read_to_string("src/day10.input").unwrap();
    let instructions = parse_instructions(&input).map_err(|err| err.to_string())?.1;

    instructions.iter().for_each(|instr| {
        draw(&mut x, cycle, register, instr);
        register = instr + register;
        cycle += instr.value();
    });

    println!(
        "{:?}",
        x.into_iter().for_each(|v| {
            let s = v.into_iter().collect::<String>();
            println!("{s}");
        })
    );
    println!("----------------------------------------------------------");
    Ok(())
}
