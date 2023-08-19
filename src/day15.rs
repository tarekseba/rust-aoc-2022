use std::{cmp::Ordering, collections::BTreeSet, fs, ops::Deref};

use nom::{
    bytes::complete::tag, character::complete::newline, combinator::map, multi::separated_list1,
    sequence::separated_pair, IResult,
};

fn parse_point_pair(input: &str) -> IResult<&str, PairCoord> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, sensor) = map(
        separated_pair(
            nom::character::complete::i32,
            tag(", y="),
            nom::character::complete::i32,
        ),
        |coords| Point {
            x: coords.0,
            y: coords.1,
        },
    )(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, beacon) = map(
        separated_pair(
            nom::character::complete::i32,
            tag(", y="),
            nom::character::complete::i32,
        ),
        |coords| Point {
            x: coords.0,
            y: coords.1,
        },
    )(input)?;
    Ok((input, PairCoord { sensor, beacon }))
}

fn parse_positions(input: &str) -> IResult<&str, BTreeSet<PairCoord>> {
    map(separated_list1(newline, parse_point_pair), |elems| {
        elems.into_iter().collect()
    })(input)
}

#[derive(Debug, Clone, PartialOrd, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.x.cmp(&other.x) {
            std::cmp::Ordering::Less => Ordering::Less,
            std::cmp::Ordering::Equal => self.y.cmp(&other.y),
            std::cmp::Ordering::Greater => Ordering::Greater,
        }
    }
}

#[derive(Debug, Eq, PartialOrd)]
struct PairCoord {
    sensor: Point,
    beacon: Point,
}

impl PartialEq for PairCoord {
    fn eq(&self, other: &Self) -> bool {
        self.sensor.eq(&other.sensor) && self.beacon.eq(&other.beacon)
    }
}

impl Ord for PairCoord {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.sensor.cmp(&other.sensor) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.beacon.cmp(&other.beacon),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PairCoord {
    fn manhattan_d(&self) -> i32 {
        (self.sensor.x - self.beacon.x).abs() + (self.sensor.y - self.beacon.y).abs()
    }

    fn add_padding(
        &self,
        padded: &mut BTreeSet<Point>,
        exclude: &BTreeSet<Point>,
        target_y_coord: i32,
    ) {
        let distance = self.manhattan_d();
        if (self.sensor.y - distance..=self.sensor.y + distance).contains(&target_y_coord) {
            let pad_number = (distance - (self.sensor.y - target_y_coord).abs()).abs();
            (self.sensor.x - pad_number..=self.sensor.x + pad_number)
                .into_iter()
                .for_each(|x| {
                    let point = Point {
                        x,
                        y: target_y_coord,
                    };
                    if !exclude.contains(&point) {
                        padded.insert(point);
                    }
                });
        }
    }
}

#[allow(unused)]
const SAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

pub fn run_part_one() -> Result<(), String> {
    // let input = SAMPLE;
    let input = fs::read_to_string("src/day15.input").map_err(|e| e.to_string())?;
    let pair_coords = parse_positions(&input)
        .map_err(|e| {
            println!("{:?}", e);
            e.to_string()
        })?
        .1;

    let exclude = pair_coords
        .iter()
        .map(|pair| vec![pair.sensor.clone(), pair.beacon.clone()])
        .flatten()
        .collect::<BTreeSet<Point>>();

    let res = pair_coords.iter().fold(BTreeSet::new(), |mut acc, pair| {
        pair.add_padding(&mut acc, &exclude, 2000000);
        acc
    });

    println!("--------------------------------- DAY 15 -----------------------------------");
    println!("Number of impossible positions is : {}", res.len());
    println!("--------------------------------- Part 2 -----------------------------------");
    Ok(())
}
