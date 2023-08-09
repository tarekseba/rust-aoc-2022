use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    ops::Sub,
};

type Grid = Vec<Vec<char>>;
type Stack = VecDeque<(usize, usize)>;
type Distances = HashMap<(usize, usize), usize>;
type Visited = HashSet<(usize, usize)>;

#[allow(unused)]
const SAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

pub fn run_part_one() -> Option<()> {
    // let input = SAMPLE;
    let input = fs::read_to_string("src/day12.input").unwrap();

    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (y_target, v): (usize, &mut Vec<char>) =
        grid.iter_mut().enumerate().find(|l| l.1.contains(&'E'))?;

    let (x_target, character) = v.iter_mut().enumerate().find(|(_, c)| *c == &'E')?;
    *character = '{';

    let (y_start, v): (usize, &mut Vec<char>) = grid
        .iter_mut()
        .enumerate()
        .find(|(_, c)| c.contains(&'S'))?;
    let (x_start, _) = v.iter_mut().enumerate().find(|(_, c)| *c == &'S')?;

    grid[y_start][x_start] = 'z';

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut distances: Distances = HashMap::new();
    let mut stack: Stack = VecDeque::from(vec![]);
    let mut unique_stack: Visited = HashSet::new();

    let (y_len, x_len) = (grid.len(), grid[0].len());

    distances.insert((x_start, y_start), 0);
    stack.push_back((x_start, y_start));

    while !stack.is_empty() {
        let point = stack.pop_front().unwrap();
        visited.insert(point);
        let distance = distances.get(&point);
        let distance = distance.unwrap();
        check_each_neighbor(
            point,
            &grid,
            distance.clone(),
            &mut stack,
            &mut unique_stack,
            &mut distances,
            &visited,
            x_len,
            y_len,
        );
    }

    println!("{:?}", distances.get(&(x_target, y_target)));
    Some(())
}

struct CharWrapper<'a>(&'a char);

trait IntoWrapper<T> {
    fn into_w(self) -> T;
}
impl<'a> IntoWrapper<CharWrapper<'a>> for &'a char {
    fn into_w(self) -> CharWrapper<'a> {
        CharWrapper(self)
    }
}

impl<'a> Sub for CharWrapper<'a> {
    type Output = i8;

    fn sub(self, rhs: Self) -> Self::Output {
        let (x, y) = (self.0, rhs.0);
        *x as i8 - *y as i8
    }
}

fn can_move_to(c1: char, c2: char) -> bool {
    c1.into_w() - c2.into_w() >= -1
}

fn handle_point(
    stack: &mut Stack,
    unique_stack: &mut Visited,
    distances: &mut Distances,
    new_coord: (usize, usize),
    distance: usize,
) {
    if unique_stack.insert(new_coord) {
        stack.push_back(new_coord);
    }
    distances
        .entry(new_coord)
        .and_modify(|entry| {
            if *entry > distance {
                *entry = distance
            }
        })
        .or_insert(distance);
}

// [(0, -1), (0, +1), (-1, 0), (+1, 0)]
fn check_each_neighbor(
    (x, y): (usize, usize),
    grid: &Grid,
    distance: usize,
    stack: &mut Stack,
    unique_stack: &mut Visited,
    distances: &mut Distances,
    visited: &Visited,
    x_len: usize,
    y_len: usize,
) {
    let distance = distance + 1;
    let current_char = grid[y][x];
    if let Some(x) = x.checked_sub(1) {
        if !visited.contains(&(x, y)) && can_move_to(current_char, grid[y][x]) {
            handle_point(stack, unique_stack, distances, (x, y), distance);
        }
    }
    if let Some(x) = x.checked_add(1) {
        if x < x_len && !visited.contains(&(x, y)) && can_move_to(current_char, grid[y][x]) {
            handle_point(stack, unique_stack, distances, (x, y), distance);
        }
    }
    if let Some(y) = y.checked_sub(1) {
        if !visited.contains(&(x, y)) && can_move_to(current_char, grid[y][x]) {
            handle_point(stack, unique_stack, distances, (x, y), distance);
        }
    }
    if let Some(y) = y.checked_add(1) {
        if y < y_len && !visited.contains(&(x, y)) && can_move_to(current_char, grid[y][x]) {
            handle_point(stack, unique_stack, distances, (x, y), distance);
        }
    }
}
