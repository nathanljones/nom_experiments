use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{space0, space1};
use nom::multi::fold_many0;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;
use nom::Parser;

const TEST_DATA: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

#[derive(Debug)]
struct Card {
    number: u32,
    first_number_set: Vec<u32>,
    second_number_set: Vec<u32>,
}

fn main() {
    dbg!(parse_data(TEST_DATA));
}

fn parse_data(data: &str) -> Vec<Card> {
    data.lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect()
}
fn parse_number_list(input: &str) -> IResult<&str, Vec<u32>> {
    fold_many0(
        terminated(complete::u32, space0),
        Vec::new,
        |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        },
    )
    .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Card> {
    let (input, _) = (tag("Card"),space1).parse(input)?;
    let (input, number) = complete::u32.parse(input)?;
    let (input, _) = (tag(":"), space1).parse(input)?;
    let (input, (first_no_list, second_number_list)) = separated_pair(
        parse_number_list,
        (tag("|"), space1),
        parse_number_list,
    )
    .parse(input)?;
    Ok((
        input,
        Card {
            number: number,
            first_number_set: first_no_list,
            second_number_set: second_number_list,
        },
    ))
}
