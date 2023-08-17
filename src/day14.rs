use std::{collections::BTreeSet, fs};

use nom::{
    bytes::complete::{is_a, tag},
    character::complete::newline,
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

type Point = (i32, i32);
type Canvas = BTreeSet<(i32, i32)>;

fn parse_pair(input: &str) -> IResult<&str, Point> {
    separated_pair(
        map_res(is_a("0123456789"), |t: &str| t.parse::<i32>()),
        tag(","),
        map_res(is_a("0123456789"), |t: &str| t.parse::<i32>()),
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(tag(" -> "), parse_pair)(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    separated_list1(newline, parse_line)(input)
}

fn build_iten(canvas: &mut Canvas, points: &Vec<Point>) {
    for i in 0..points.len() - 1 {
        let (x1, y1) = points[i];
        let (x2, y2) = points[i + 1];
        if x1 != x2 {
            let (x1, x2) = (x1.min(x2), x1.max(x2));
            for j in 0..=(x2 - x1) {
                canvas.insert((x1 + j, y1));
            }
        } else if y1 != y2 {
            let (y1, y2) = (y1.min(y2), y1.max(y2));
            for j in 0..=(y2 - y1) {
                canvas.insert((x1, y1 + j));
            }
        }
    }
}

fn can_move(
    canvas: &Canvas,
    (x, y): Point,
    predicate: impl Fn(bool, &i32) -> bool,
) -> Option<Point> {
    if predicate(canvas.get(&(x, y + 1)).is_none(), &y) {
        return Some((x, y + 1));
    } else if predicate(canvas.get(&(x - 1, y + 1)).is_none(), &y) {
        return Some((x - 1, y + 1));
    } else if predicate(canvas.get(&(x + 1, y + 1)).is_none(), &y) {
        return Some((x + 1, y + 1));
    }
    None
}

fn play(canvas: &mut Canvas, max_depth: i32, predicate: impl Fn(bool, &i32) -> bool) -> Option<()> {
    let mut start = (500, 0);
    while let Some((x, y)) = can_move(canvas, start, &predicate) {
        if y >= max_depth {
            return Some(());
        }
        start = (x, y);
    }
    canvas.insert(start);
    None
}

fn play_bottom(canvas: &mut Canvas, predicate: impl Fn(bool, &i32) -> bool) -> Option<()> {
    let mut start = (500, 0);
    while let Some((x, y)) = can_move(canvas, start, &predicate) {
        start = (x, y);
    }
    canvas.insert(start);
    if start.1 == 0 {
        return Some(());
    }
    None
}

#[allow(unused)]
const SAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

pub fn run_part_one() -> Result<(), String> {
    // let input = SAMPLE;
    let input = fs::read_to_string("src/day14.input").map_err(|e| e.to_string())?;
    let lines = parse_lines(&input).map_err(|err| err.to_string())?.1;

    let mut canvas: Canvas = Canvas::new();
    lines
        .iter()
        .for_each(|vec_of_pairs| build_iten(&mut canvas, vec_of_pairs));

    let start_points = canvas.len();
    let max_depth = canvas
        .iter()
        .map(|x| x.1)
        .max()
        .expect("failed to find max_depth");

    let predicate = move |val: bool, _y: &i32| val;

    loop {
        match play(&mut canvas, max_depth, predicate) {
            Some(_) => break,
            None => continue,
        }
    }

    println!("--------------------------------- DAY 14 -----------------------------------");
    println!(
        "max number of sand points is : {}",
        canvas.len() - start_points
    );
    println!("--------------------------------- PART 2 -----------------------------------");
    Ok(())
}

pub fn run_part_two() -> Result<(), String> {
    // let input = SAMPLE;
    let input = fs::read_to_string("src/day14.input").map_err(|e| e.to_string())?;
    let lines = parse_lines(&input).map_err(|err| err.to_string())?.1;

    let mut canvas: Canvas = Canvas::new();
    lines
        .iter()
        .for_each(|vec_of_pairs| build_iten(&mut canvas, vec_of_pairs));

    let start_points = canvas.len();
    let max_depth = canvas
        .iter()
        .map(|x| x.1)
        .max()
        .expect("failed to find max_depth");

    let predicate = move |val: bool, y: &i32| val && y + 1 < max_depth + 2;

    loop {
        match play_bottom(&mut canvas, predicate) {
            Some(_) => break,
            None => continue,
        }
    }

    println!(
        "max number of sand points with floor is : {}",
        canvas.len() - start_points
    );
    println!("----------------------------------------------------------------------------");
    Ok(())
}
