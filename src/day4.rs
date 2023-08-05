use std::{fs, ops::RangeInclusive, str::FromStr};

#[allow(unused)]
const SAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

#[derive(Debug)]
struct RangeWrapper<T>(RangeInclusive<T>);

pub fn run_part_one() {
    // let overlapping_ranges = SAMPLE
    let overlapping_ranges = fs::read_to_string("src/day4.input")
        .unwrap()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|range| range.parse::<RangeWrapper<u32>>().unwrap())
                .collect::<Vec<RangeWrapper<u32>>>()
        })
        .map(|ranges| ranges[0].overlap(&ranges[1]) || ranges[1].overlap(&ranges[0]))
        .fold(0, |acc, el| acc + if el { 1 } else { 0 });
    println!("{overlapping_ranges}")
}

pub fn run_part_two() {
    // let overlapping_ranges = SAMPLE
    let overlapping_ranges = fs::read_to_string("src/day4.input")
        .unwrap()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|range| range.parse::<RangeWrapper<u32>>().unwrap())
                .collect::<Vec<RangeWrapper<u32>>>()
        })
        .map(|ranges| ranges[0].partial_overlap(&ranges[1]))
        .fold(0, |acc, el| acc + if el { 1 } else { 0 });
    println!("{overlapping_ranges}")
}

impl FromStr for RangeWrapper<u32> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec_tuple: Vec<u32> = s
            .split("-")
            .map(|value| value.parse::<u32>().map_err(|err| err.to_string()))
            .collect::<Result<Vec<u32>, String>>()?;
        Ok(RangeWrapper(vec_tuple[0]..=vec_tuple[1]))
    }
}

trait Overlap {
    fn overlap(&self, t: &Self) -> bool;
}

impl Overlap for RangeWrapper<u32> {
    fn overlap(&self, t: &Self) -> bool {
        self.0.contains(&t.0.start()) && self.0.contains(t.0.end())
    }
}

trait PartialOverlap {
    fn partial_overlap(&self, t: &Self) -> bool;
}

impl PartialOverlap for RangeWrapper<u32> {
    fn partial_overlap(&self, t: &Self) -> bool {
        self.0.contains(t.0.end())
            || self.0.contains(t.0.start())
            || t.0.contains(self.0.end())
            || t.0.contains(self.0.start())
    }
}
