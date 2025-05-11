use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::sequence::terminated;
use nom::{IResult, Parser};
use nom::multi::separated_list1;

const TEST_DATA: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
#[derive(Debug)]
struct UVec2 {
    pub x: u32,
    pub y: u32,
}
#[derive(Debug)]
struct Machine {
    a: UVec2,
    b: UVec2,
    prize: UVec2,
}
fn main() {
    let res = parse(TEST_DATA);
    dbg!(res);
}

fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1((line_ending,line_ending), machine,).parse(input)
}
fn parse_x_y<'a>(input: &'a str,sign:&'a str) -> IResult<&'a str, UVec2> {
    let (input, _) = (tag("X"),tag(sign)).parse(input)?;
    let (input, x) = complete::u32.parse(input)?;
    let (input, _) = (tag(", Y"),tag(sign)).parse(input)?;
    let (input, y) = complete::u32.parse(input)?;

    Ok((input, UVec2 { x, y }))
}

fn a_button(input: &str) -> IResult<&str, UVec2> {
    let (input, _) = tag("Button A: ").parse(input)?;
    let (input, xy) = parse_x_y(input,"+")?;
    Ok((input, xy))
}
fn b_button(input: &str) -> IResult<&str, UVec2> {
    let (input, _) = tag("Button B: ").parse(input)?;
    let (input, xy) = parse_x_y(input,"+")?;
    Ok((input, xy))
}

fn prize(input: &str) -> IResult<&str, UVec2> {
    let (input, _) = tag("Prize: ").parse(input)?;
    let (input, xy) = parse_x_y(input,"=")?;
    Ok((input, xy))
}
fn machine(input: &str) -> IResult<&str, Machine> {
    let (input, (a, b, p)) = (
        terminated(a_button, line_ending),
        terminated(b_button, line_ending),
        prize,
    )
        .parse(input)?;
    Ok((input, Machine { a, b, prize: p }))
}

mod test {
    use super::*;
    #[test]
    fn test_parse_x_y() {
        let inp = "X+22, Y+67";
        let res = parse_x_y(inp,"+");
        dbg!(res);
    }
    #[test]
    fn test_machine() {
        let inp = "Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450";
        let res = machine(inp);
        dbg!(res);
    }
}
