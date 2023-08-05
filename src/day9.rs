use std::{collections::HashSet, fs, str::FromStr};

use nom::{
    bytes::complete::is_a, character::complete::newline, combinator::map_res,
    multi::separated_list0, IResult,
};

#[derive(Debug, Default)]
enum Direction {
    #[default]
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Default)]
struct Move(Direction, usize);

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(" ").collect::<Vec<&str>>();
        let step_size = s[1].parse::<usize>().map_err(|err| err.to_string())?;
        match s[0] {
            "U" => Ok(Move(Direction::UP, step_size)),
            "D" => Ok(Move(Direction::DOWN, step_size)),
            "L" => Ok(Move(Direction::LEFT, step_size)),
            "R" => Ok(Move(Direction::RIGHT, step_size)),
            _ => Err("Wrong input".into()),
        }
    }
}

impl std::ops::Add<i16> for Direction {
    type Output = i16;

    fn add(self, rhs: i16) -> Self::Output {
        match self {
            Direction::UP => rhs + 1,
            Direction::DOWN => rhs - 1,
            Direction::LEFT => rhs - 1,
            Direction::RIGHT => rhs + 1,
        }
    }
}

impl std::ops::Add<&Direction> for (i16, i16) {
    type Output = Self;

    fn add(self, rhs: &Direction) -> Self::Output {
        match rhs {
            Direction::LEFT => (self.0 - 1, self.1),
            Direction::RIGHT => (self.0 + 1, self.1),
            Direction::UP => (self.0, self.1 + 1),
            Direction::DOWN => (self.0, self.1 - 1),
        }
    }
}

#[derive(Default, Debug)]
struct Playground {
    head: (i16, i16),
    tail: (i16, i16),
    visited: HashSet<(i16, i16)>,
}

impl Playground {
    fn next_move(&self) -> (Option<Direction>, Option<Direction>) {
        let (x, y) = self.distance();
        if (-1..=1).contains(&(x.abs() + y.abs())) && x != 0 && y != 0 {
            (None, None)
        } else {
            let x_num = match x {
                number if number > 1 => Some(Direction::RIGHT),
                number if number < -1 => Some(Direction::LEFT),
                number if number == 1 && !(-1..=1).contains(&y) => Some(Direction::RIGHT),
                number if number == -1 && !(-1..=1).contains(&y) => Some(Direction::LEFT),
                _ => None,
            };
            let y_num = match y {
                number if number > 1 => Some(Direction::UP),
                number if number < -1 => Some(Direction::DOWN),
                number if number == 1 && !(-1..=1).contains(&x) => Some(Direction::UP),
                number if number == -1 && !(-1..=1).contains(&x) => Some(Direction::DOWN),
                _ => None,
            };
            (x_num, y_num)
        }
    }

    fn move_play(&mut self, move_dir: Move) {
        for _ in 0..move_dir.1 {
            self.head = self.head + &move_dir.0;
            let next_move = self.next_move();
            self.tail = (
                next_move
                    .0
                    .map_or(self.tail.0, |number: Direction| number + self.tail.0),
                next_move
                    .1
                    .map_or(self.tail.1, |number: Direction| number + self.tail.1),
            );
            self.visited.insert(self.tail);
        }
    }

    fn distance(&self) -> (i16, i16) {
        (self.head.0 - self.tail.0, self.head.1 - self.tail.1)
    }
}

#[allow(unused)]
const SAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

fn parse_move(input: &str) -> IResult<&str, Move> {
    map_res(is_a("UDRL 1234567890"), |res: &str| res.parse::<Move>())(input)
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list0(newline, parse_move)(input)
}

pub fn run_part_one() -> Result<(), String> {
    let mut x = Playground::default();
    // let input = SAMPLE;
    let input = fs::read_to_string("src/day9.input").unwrap();
    let moves = parse_moves(&input).map_err(|err| err.to_string())?.1;
    moves.into_iter().for_each(|mov: Move| x.move_play(mov));
    println!("visited knots : {}\n", x.visited.len());
    Ok(())
}
