use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::Parser;
use nom::{IResult};

const TEST_DATA: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

#[derive(Debug)]
struct Holder {
    first_number: u32,
    second_number: u32,
}

fn main() {
    dbg!(parse_data(TEST_DATA));
}
fn parse_data(data: &str) -> Vec<Holder> {
    data.lines().map(|line| parse_number(line).unwrap().1).collect()
}

fn parse_number(input: &str) -> IResult<&str, Holder> {
    let (input, first) = digit1.parse(input)?;
    let (input, _) = multispace0.parse(input)?;
    let (input, second) = digit1.parse(input)?;
    
    Ok((
        input,
        Holder {
            first_number: first.parse::<u32>().unwrap(),
            second_number: second.parse::<u32>().unwrap(),
        },
    ))
}

mod test {
    use super::*;
    #[test]
    fn test_parse_number(){
        let ret = parse_number("123 456");
        assert!(ret.is_ok());
        if let Ok((_, v)) =  parse_number("123 456"){
            assert_eq!(v.first_number, 123);
            assert_eq!(v.second_number, 456);
        }
        
    }

    

}
