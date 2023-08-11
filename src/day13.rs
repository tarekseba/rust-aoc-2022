use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::{map, opt},
    multi::{many0, many1},
    sequence::preceded,
    IResult,
};

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

#[derive(Debug)]
enum List {
    Value(u32),
    Cons(Vec<List>),
}

fn parse_values(input: &str) -> IResult<&str, Vec<List>> {
    println!("inside parse_values => input : {input}");
    let (input, val) = many0(preceded(
        opt(tag(",")),
        alt((map(complete::u32, |i| List::Value(i)), parse_list)),
    ))(input)?;
    Ok((input, val))
}

fn parse_list(input: &str) -> IResult<&str, List> {
    println!("inside parse_list => input   : {input}");
    let (input, _) = tag("[")(input)?;
    let (input, values) = parse_values(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, List::Cons(values)))
}

pub fn run_part_one() -> Result<(), String> {
    let input = SAMPLE;
    let splitted_input = input.split("\n\n").collect::<Vec<&str>>();
    println!("{:?}", splitted_input);

    let entries = splitted_input
        .iter()
        .map(|pair| {
            pair.split("\n")
                .map(|e| {
                    parse_list(e)
                        .map_err(|err| err.to_string())
                        .map(|(input, el)| el)
                })
                .collect::<Result<Vec<List>, String>>()
        })
        .collect::<Result<Vec<Vec<List>>, String>>()?;

    entries.iter().for_each(|el| {
        println!("\n");
        el.iter().for_each(|e| println!("{:?}", e));
    });

    let x = &entries[1];
    let (x, y) = (&x[0], &x[1]);
    match x {
        List::Cons(v1) => match y {
            List::Cons(v2) => v1.iter().zip(v2.iter()).for_each(|e| println!("{:?}", e)),
            _ => panic!("F to pay respect"),
        },
        _ => panic!("F to pay respect"),
    }
    Ok(())
}
