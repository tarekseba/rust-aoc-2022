use std::{collections::HashMap, fs, str::FromStr};

const SAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

pub fn run_part_one() -> Result<(), String> {
    let mut column_map: HashMap<u32, Vec<String>> = HashMap::new();
    let parts = fs::read_to_string("src/day5.input").unwrap();
    let parts = parts.split("\n\n").collect::<Vec<&str>>();
    let (placement, moves) = (parts[0], parts[1]);
    let mut placements = placement.lines().collect::<Vec<&str>>();
    placements.pop();
    let placements = placements
        .iter()
        .map(|line| parse_line(line))
        .collect::<Vec<Vec<Option<String>>>>();
    placements.into_iter().for_each(|line_vec| {
        line_vec
            .into_iter()
            .enumerate()
            .for_each(|(index, element)| {
                if element.is_some() {
                    let element = element.unwrap();
                    let cloned_element = element.clone();
                    column_map
                        .entry((index + 1) as u32)
                        .and_modify(|vec_val| vec_val.push(element))
                        .or_insert(vec![cloned_element]);
                }
            })
    });
    column_map.iter_mut().for_each(|(_, value)| value.reverse());

    let moves = moves
        .lines()
        .map(|line| line.parse::<Move>())
        .collect::<Result<Vec<Move>, String>>()?;

    moves.iter().for_each(|mov| mov.execute(&mut column_map));
    let mut res = column_map
        .iter()
        .map(|(key, value)| (*key, value.last()))
        .collect::<Vec<(u32, Option<&String>)>>();
    res.sort();
    let res = res.into_iter().fold(String::from(""), |mut acc, val| {
        acc.push_str(val.1.unwrap());
        acc
    });
    println!("{:?}", res.replace("[", "").replace("]", ""));
    Ok(())
}

pub fn run_part_two() -> Result<(), String> {
    let mut column_map: HashMap<u32, Vec<String>> = HashMap::new();
    let parts = fs::read_to_string("src/day5.input").unwrap();
    let parts = parts.split("\n\n").collect::<Vec<&str>>();
    let (placement, moves) = (parts[0], parts[1]);
    let mut placements = placement.lines().collect::<Vec<&str>>();
    placements.pop();
    let placements = placements
        .iter()
        .map(|line| parse_line(line))
        .collect::<Vec<Vec<Option<String>>>>();
    placements.into_iter().for_each(|line_vec| {
        line_vec
            .into_iter()
            .enumerate()
            .for_each(|(index, element)| {
                if element.is_some() {
                    let element = element.unwrap();
                    let cloned_element = element.clone();
                    column_map
                        .entry((index + 1) as u32)
                        .and_modify(|vec_val| vec_val.push(element))
                        .or_insert(vec![cloned_element]);
                }
            })
    });
    column_map.iter_mut().for_each(|(_, value)| value.reverse());

    let moves = moves
        .lines()
        .map(|line| line.parse::<Move>())
        .collect::<Result<Vec<Move>, String>>()?;

    moves
        .iter()
        .for_each(|mov| mov.execute_9001(&mut column_map));
    let mut res = column_map
        .iter()
        .map(|(key, value)| (*key, value.last()))
        .collect::<Vec<(u32, Option<&String>)>>();
    res.sort();
    let res = res.into_iter().fold(String::from(""), |mut acc, val| {
        acc.push_str(val.1.unwrap());
        acc
    });
    println!("{:?}", res.replace("[", "").replace("]", ""));
    Ok(())
}

#[derive(Debug)]
struct Move(u32, u32, u32);

impl Move {
    fn execute(&self, placements: &mut HashMap<u32, Vec<String>>) {
        let Move(count, from, to) = self;
        let _ = placements
            .get_mut(from)
            .and_then(|from_col| {
                let count = (*count).min(from_col.len() as u32);
                let vec = from_col.split_off(from_col.len() - count as usize);
                Some(vec)
            })
            .and_then(|mut vec| {
                placements.get_mut(to).and_then(|vec_to| {
                    vec.reverse();
                    vec_to.append(&mut vec);
                    Some(())
                });
                Some(())
            });
    }

    fn execute_9001(&self, placements: &mut HashMap<u32, Vec<String>>) {
        let Move(count, from, to) = self;
        let _ = placements
            .get_mut(from)
            .and_then(|from_col| {
                let count = (*count).min(from_col.len() as u32);
                let vec = from_col.drain(from_col.len() - count as usize..).collect();
                Some(vec)
            })
            .and_then(|mut vec| {
                placements.get_mut(to).and_then(|vec_to| {
                    vec_to.append(&mut vec);
                    Some(())
                });
                Some(())
            });
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s
            .split(" ")
            .filter(|item| {
                !item.is_empty() && !item.contains(|character: char| character.is_alphabetic())
            })
            .map(|character| {
                character
                    .parse::<u32>()
                    .map_err(|err| format!("Badly formatted move : {}", err.to_string()))
            })
            .collect::<Result<Vec<u32>, String>>()?;
        Ok(Move(x[0], x[1], x[2]))
    }
}

fn parse_line(line: &str) -> Vec<Option<String>> {
    let mut index = 0;
    let mut blocks: Vec<Option<String>> = vec![];
    while index < line.len() {
        let iter = &line[index..=index + 2];
        index += 4;
        if iter.trim().len() == 0 {
            blocks.push(None)
        } else {
            blocks.push(Some(iter.to_string()))
        }
    }
    blocks
}
