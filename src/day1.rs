use anyhow::Error;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::bytes::complete::take_while_m_n;
use nom::character::complete::digit1;
use nom::multi::many0;
use nom::IResult;
use std::fs::read_to_string;

pub fn calculate() -> Result<(String, String), Error> {
    let input = read_to_string("input/day1")?;

    let parsed_input = parse_input(&input)?;
    let parsed_input_second = parse_input_second(&input)?;

    Ok((add(parsed_input), add(parsed_input_second)))
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}
fn is_not_digit(c: char) -> bool {
    c.is_alphabetic()
}

fn is_digit_or_number_name(input: &str) -> IResult<&str, &str> {
    alt((
        tag("oneight"),
        tag("twone"),
        tag("threeight"),
        tag("fiveight"),
        tag("sevenine"),
        tag("eightwo"),
        tag("eighthree"),
        tag("nineight"),
        tag("one"),
        tag("two"),
        tag("three"),
        tag("four"),
        tag("five"),
        tag("six"),
        tag("seven"),
        tag("eight"),
        tag("nine"),
        digit1,
    ))(input)
}

fn a_digit(input: &str) -> IResult<&str, u32> {
    let (rest, _) = take_while(is_not_digit)(input)?;
    let (rest, data) = take_while_m_n(1, 1, is_digit)(rest)?;
    let (rest, _) = take_while(is_not_digit)(rest)?;

    Ok((rest, data.parse().unwrap()))
}

fn row(input: &str) -> IResult<&str, Vec<u32>> {
    let (rest, data) = many0(a_digit)(input)?;
    let (rest, _) = tag("\n")(rest)?;

    Ok((rest, data.into_iter().map(|d| d.to_owned()).collect()))
}

fn multi(i: &str) -> IResult<&str, Vec<Vec<u32>>> {
    many0(row)(i)
}

fn parse_input(input: &str) -> Result<Vec<(u32, u32)>, Error> {
    let (_, result) = multi(input).map_err(|e| e.to_owned())?;

    Ok(result.into_iter().map(|v| (v[0], v[v.len() - 1])).collect())
}

fn convert_number(input: &str) -> Vec<u32> {
    match input {
        "oneight" => vec![1, 8],
        "twone" => vec![2, 1],
        "threeight" => vec![3, 8],
        "fiveight" => vec![5, 8],
        "sevenine" => vec![7, 9],
        "eightwo" => vec![8, 2],
        "eighthree" => vec![8, 3],
        "nineight" => vec![9, 8],
        "one" => vec![1],
        "two" => vec![2],
        "three" => vec![3],
        "four" => vec![4],
        "five" => vec![5],
        "six" => vec![6],
        "seven" => vec![7],
        "eight" => vec![8],
        "nine" => vec![9],
        any => any.chars().map(|c| c.to_digit(10).unwrap()).collect(),
    }
}

fn maybe_number(input: &str) -> IResult<&str, Option<Vec<u32>>> {
    match is_digit_or_number_name(input) {
        Ok((rest, data)) => Ok((rest, Some(convert_number(data)))),
        Err(_err) => {
            let (rest, _data) = take_while_m_n(1, 1, |c| c != '\n')(input)?;
            Ok((rest, None))
        }
    }
}

fn row_second(input: &str) -> IResult<&str, Vec<u32>> {
    let (rest, data) = many0(maybe_number)(input)?;
    let (rest, _) = tag("\n")(rest)?;

    let data = data
        .into_iter()
        .filter(|o| o.is_some())
        .flat_map(|d| d.unwrap().into_iter())
        .collect();
    Ok((rest, data))
}

fn multi_second(i: &str) -> IResult<&str, Vec<Vec<u32>>> {
    many0(row_second)(i)
}

fn parse_input_second(input: &str) -> Result<Vec<(u32, u32)>, Error> {
    let (_, result) = multi_second(input).map_err(|e| e.to_owned())?;

    Ok(result.into_iter().map(|v| (v[0], v[v.len() - 1])).collect())
}

fn add(data: Vec<(u32, u32)>) -> String {
    format!("{}", data.iter().fold(0, |acc, d| acc + 10 * d.0 + d.1))
}

#[test]
pub fn test_parse() {
    let res = parse_input(
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
",
    );

    assert_eq!(vec![(1, 2), (3, 8), (1, 5), (7, 7)], res.unwrap());
}

#[test]
pub fn test_parse_second() {
    let res = parse_input_second(
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
",
    );

    assert_eq!(
        vec![(2, 9), (8, 3), (1, 3), (2, 4), (4, 2), (1, 4), (7, 6)],
        res.unwrap()
    );
}

#[test]
pub fn test_parse_second_greedy1() {
    let res = parse_input_second("1sevenine\n");

    assert_eq!(vec![(1, 9)], res.unwrap());
}

#[test]
pub fn test_parse_second_greedy2() {
    let res = parse_input_second("sevenine\n");

    assert_eq!(vec![(7, 9)], res.unwrap());
}

#[test]
pub fn test_parse_second_greedy3() {
    let res = parse_input_second("eightwothree\n");

    assert_eq!(vec![(8, 3)], res.unwrap());
}

#[test]
pub fn test_first() {
    assert_eq!("142", add(vec![(1, 2), (3, 8), (1, 5), (7, 7)]));
}

#[test]
pub fn test_second() {
    assert_eq!(
        "281",
        add(vec![(2, 9), (8, 3), (1, 3), (2, 4), (4, 2), (1, 4), (7, 6)])
    );
}
