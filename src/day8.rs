use std::{fs, num::ParseIntError, str::FromStr};

const SAMPLE: &str = "30373
25512
65332
33549
35390";

struct Forest(Vec<Vec<u8>>);

enum Sight {
    Top,
    Bottom,
    Left,
    Right,
}

impl std::fmt::Debug for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().for_each(|inner| {
            inner.iter().enumerate().for_each(|(index, number)| {
                let _ = write!(
                    f,
                    "{number}{}",
                    if index < (inner.len() - 1) { " " } else { "" }
                );
            });
            let _ = write!(f, "\n");
        });
        Ok(())
    }
}

impl Forest {
    fn get_line(&self, sight: &Sight, line_index: usize, col_index: usize) -> Option<Vec<u8>> {
        match sight {
            Sight::Left | Sight::Right => match self.0.get(line_index) {
                None => None,
                Some(vec) => Some(vec.clone()),
            },
            Sight::Top | Sight::Bottom => self
                .0
                .iter()
                .map(|el| el.get(col_index).map(|number| number.clone()))
                .collect::<Option<Vec<u8>>>(),
        }
    }

    fn count_visible_trees(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(line_index, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(col_index, _)| {
                        if col_index == 0
                            || line_index == 0
                            || col_index == line.len() - 1
                            || line_index == self.0.len() - 1
                        {
                            true
                        } else {
                            vec![
                                self.is_visible_trees(Sight::Top, line_index, col_index),
                                self.is_visible_trees(Sight::Bottom, line_index, col_index),
                                self.is_visible_trees(Sight::Left, line_index, col_index),
                                self.is_visible_trees(Sight::Right, line_index, col_index),
                            ]
                            .iter()
                            .flatten()
                            .filter(|&&v| v)
                            .count()
                                > 0
                        }
                    })
                    .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>()
            .iter()
            .flatten()
            .filter(|v| **v)
            .count()
    }

    fn is_visible_trees(
        &self,
        sight: Sight,
        line_index: usize,
        col_index: usize,
    ) -> Result<bool, String> {
        let line = self
            .get_line(&sight, line_index, col_index)
            .ok_or(format!("line or column not found for index {line_index}"))?;
        let target = match sight {
            Sight::Top | Sight::Bottom => line.get(line_index).unwrap(),
            Sight::Left | Sight::Right => line.get(col_index).unwrap(),
        };
        let res: bool = match sight {
            Sight::Left => {
                line.iter()
                    .enumerate()
                    .take_while(|(index, val)| **val < *target && *index < col_index)
                    .count()
                    >= col_index
            }
            Sight::Right => {
                let val_index = transpose_index(col_index, line.len());
                line.iter()
                    .rev()
                    .enumerate()
                    .take_while(|(index, val)| **val < *target && *index < val_index)
                    .count()
                    >= val_index
            }
            Sight::Top => {
                line.iter()
                    .enumerate()
                    .take_while(|(index, val)| **val < *target && *index < line_index)
                    .count()
                    >= line_index
            }
            Sight::Bottom => {
                let val_index = transpose_index(line_index, line.len());
                line.iter()
                    .rev()
                    .enumerate()
                    .take_while(|(index, val)| **val < *target && *index < val_index)
                    .count()
                    >= val_index
            }
        };
        Ok(res)
    }

    fn tree_score(&self, line_index: usize, col_index: usize) -> Result<usize, String> {
        if line_index == 0 || col_index == 0 {
            Ok(0)
        } else {
            let line_top = self
                .get_line(&Sight::Top, line_index, col_index)
                .ok_or(format!("line or column not found for index {line_index}"))?;
            let line_sides = self
                .get_line(&Sight::Left, line_index, col_index)
                .ok_or(format!("line or column not found for index {line_index}"))?;
            let (target_top, target_sides) = (
                line_top.get(line_index).unwrap(),
                line_sides.get(col_index).unwrap(),
            );
            let score_right_left = {
                let val_index = transpose_index(col_index, line_sides.len());
                let mut left_scores = line_sides
                    .iter()
                    .skip(col_index + 1)
                    .take_while(|&v| v < target_sides)
                    .count();
                let mut right_scores = line_sides
                    .iter()
                    .skip(val_index + 1)
                    .take_while(|&v| v < target_sides)
                    .count();
                if left_scores + col_index + 1 <= line_sides.len() - 1 {
                    left_scores += 1
                }
                if right_scores + val_index + 1 <= line_sides.len() - 1 {
                    right_scores += 1
                }
                left_scores * right_scores
            };
            let score_top_bottom = {
                let val_index = transpose_index(line_index, line_top.len());
                let mut top_scores = line_top
                    .iter()
                    .skip(line_index + 1)
                    .take_while(|&v| v < target_top)
                    .count();
                let mut bottom_scores = line_top
                    .iter()
                    .rev()
                    .skip(val_index + 1)
                    .take_while(|&v| v < target_top)
                    .count();
                if top_scores + line_index + 1 <= line_top.len() - 1 {
                    top_scores += 1
                }
                if bottom_scores + val_index + 1 <= line_top.len() - 1 {
                    bottom_scores += 1
                }
                top_scores * bottom_scores
            };
            Ok(score_top_bottom * score_right_left)
        }
    }

    fn highest_score(&self) -> usize {
        let res = self
            .0
            .iter()
            .enumerate()
            .map(|(line_index, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(col_index, _)| self.tree_score(line_index, col_index))
                    .collect::<Result<Vec<usize>, String>>()
            })
            .collect::<Result<Vec<Vec<usize>>, String>>()
            .unwrap();
        *res.iter().flatten().max().unwrap()
    }
}

fn transpose_index(index: usize, length: usize) -> usize {
    length - 1 - index
}

impl FromStr for Forest {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let inner = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|character| character.to_string().parse::<u8>())
                    .collect::<Result<Vec<u8>, ParseIntError>>()
            })
            .collect::<Result<Vec<Vec<u8>>, ParseIntError>>()?;
        Ok(Forest(inner))
    }
}

pub fn run_both_parts() -> Result<(), ParseIntError> {
    // let input = SAMPLE;
    let input = fs::read_to_string("src/day8.input").unwrap();
    let forest = input.parse::<Forest>()?;
    println!("Visible trees : {:?}", forest.count_visible_trees());
    println!("Highest tree score : {:?}", forest.highest_score());
    Ok(())
}
