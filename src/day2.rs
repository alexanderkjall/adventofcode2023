use anyhow::anyhow;
use anyhow::Error;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while_m_n;
use nom::character::complete::digit1;
use nom::multi::many0;
use nom::IResult;
use std::fs::read_to_string;

pub fn calculate() -> Result<(String, String), Error> {
    let input = read_to_string("input/day2")?;

    let parsed_input: Vec<Game> = parse_input(&input).unwrap();

    let first: u32 = calculate_possible(&parsed_input, 12, 13, 14);
    let second: u32 = calculate_minimal(&parsed_input);

    Ok((format!("{}", first), format!("{}", second)))
}

fn calculate_possible(input: &[Game], limit_red: u32, limit_green: u32, limit_blue: u32) -> u32 {
    input
        .iter()
        .filter(|g| g.allowed(limit_red, limit_green, limit_blue))
        .fold(0, |acc, g| acc + g.id)
}

fn calculate_minimal(input: &[Game]) -> u32 {
    input
        .iter()
        .map(calculate_minimal_game)
        .map(|t| t.0 * t.1 * t.2)
        .sum()
}

fn calculate_minimal_game(input: &Game) -> (u32, u32, u32) {
    (
        input.hands.iter().map(|h| h.red).max().unwrap_or(0),
        input.hands.iter().map(|h| h.green).max().unwrap_or(0),
        input.hands.iter().map(|h| h.blue).max().unwrap_or(0),
    )
}

#[derive(PartialEq, Debug)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
}

#[derive(PartialEq, Debug)]
struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

impl Hand {
    fn from3(
        color1_amount: &str,
        color1: &str,
        color2_amount: &str,
        color2: &str,
        color3_amount: &str,
        color3: &str,
    ) -> Result<Self, Error> {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        match color1 {
            "red" => red = color1_amount.parse()?,
            "green" => green = color1_amount.parse()?,
            "blue" => blue = color1_amount.parse()?,
            _ => Err(anyhow!(format!("unknown color1: {}", color1)))?,
        }
        match color2 {
            "red" => red = color2_amount.parse()?,
            "green" => green = color2_amount.parse()?,
            "blue" => blue = color2_amount.parse()?,
            _ => Err(anyhow!(format!("unknown color2: {}", color1)))?,
        }
        match color3 {
            "red" => red = color3_amount.parse()?,
            "green" => green = color3_amount.parse()?,
            "blue" => blue = color3_amount.parse()?,
            _ => Err(anyhow!(format!("unknown color3: {}", color1)))?,
        }

        Ok(Self { red, green, blue })
    }

    fn from2(
        color1_amount: &str,
        color1: &str,
        color2_amount: &str,
        color2: &str,
    ) -> Result<Self, Error> {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        match color1 {
            "red" => red = color1_amount.parse()?,
            "green" => green = color1_amount.parse()?,
            "blue" => blue = color1_amount.parse()?,
            _ => Err(anyhow!(format!("unknown color1: {}", color1)))?,
        }
        match color2 {
            "red" => red = color2_amount.parse()?,
            "green" => green = color2_amount.parse()?,
            "blue" => blue = color2_amount.parse()?,
            _ => Err(anyhow!(format!("unknown color2: {}", color1)))?,
        }

        Ok(Self { red, green, blue })
    }

    fn from1(color1_amount: &str, color1: &str) -> Result<Self, Error> {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        match color1 {
            "red" => red = color1_amount.parse()?,
            "green" => green = color1_amount.parse()?,
            "blue" => blue = color1_amount.parse()?,
            _ => Err(anyhow!(format!("unknown color1: {}", color1)))?,
        }

        Ok(Self { red, green, blue })
    }
}

impl Game {
    fn allowed(&self, limit_red: u32, limit_green: u32, limit_blue: u32) -> bool {
        self.hands
            .iter()
            .all(|h| h.red <= limit_red && h.green <= limit_green && h.blue <= limit_blue)
    }
}

fn hand3(input: &str) -> IResult<&str, Hand> {
    let (rest, _) = tag(" ")(input)?;
    let (rest, color1_amount) = digit1(rest)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, color1) = alt((tag("red"), tag("green"), tag("blue")))(rest)?;

    let (rest, _) = tag(", ")(rest)?;
    let (rest, color2_amount) = digit1(rest)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, color2) = alt((tag("red"), tag("green"), tag("blue")))(rest)?;

    let (rest, _) = tag(", ")(rest)?;
    let (rest, color3_amount) = digit1(rest)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, color3) = alt((tag("red"), tag("green"), tag("blue")))(rest)?;

    let (rest, _) = take_while_m_n(0, 1, |c| c == ';')(rest)?;
    Ok((
        rest,
        Hand::from3(
            color1_amount,
            color1,
            color2_amount,
            color2,
            color3_amount,
            color3,
        )
        .unwrap(),
    ))
}

fn hand2(input: &str) -> IResult<&str, Hand> {
    let (rest, _) = tag(" ")(input)?;
    let (rest, color1_amount) = digit1(rest)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, color1) = alt((tag("red"), tag("green"), tag("blue")))(rest)?;

    let (rest, _) = tag(", ")(rest)?;
    let (rest, color2_amount) = digit1(rest)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, color2) = alt((tag("red"), tag("green"), tag("blue")))(rest)?;

    let (rest, _) = take_while_m_n(0, 1, |c| c == ';')(rest)?;
    Ok((
        rest,
        Hand::from2(color1_amount, color1, color2_amount, color2).unwrap(),
    ))
}

fn hand1(input: &str) -> IResult<&str, Hand> {
    let (rest, _) = tag(" ")(input)?;
    let (rest, color1_amount) = digit1(rest)?;
    let (rest, _) = tag(" ")(rest)?;
    let (rest, color1) = alt((tag("red"), tag("green"), tag("blue")))(rest)?;

    let (rest, _) = take_while_m_n(0, 1, |c| c == ';')(rest)?;
    Ok((rest, Hand::from1(color1_amount, color1).unwrap()))
}

fn row(input: &str) -> IResult<&str, Game> {
    let (rest, _) = tag("Game ")(input)?;
    let (rest, id) = digit1(rest)?;
    let (rest, _) = tag(":")(rest)?;
    let (rest, hands) = many0(alt((hand3, hand2, hand1)))(rest)?;
    let (rest, _) = tag("\n")(rest)?;

    Ok((
        rest,
        Game {
            id: id.parse().unwrap(),
            hands,
        },
    ))
}

fn multi(i: &str) -> IResult<&str, Vec<Game>> {
    many0(row)(i)
}

fn parse_input(input: &str) -> Result<Vec<Game>, Error> {
    let (_, result) = multi(input).map_err(|e| e.to_owned())?;

    Ok(result)
}

#[test]
pub fn test_parse() {
    let res = parse_input(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
",
    );

    assert_eq!(
        vec![
            Game {
                id: 1,
                hands: vec![
                    Hand {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    Hand {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    Hand {
                        red: 0,
                        green: 2,
                        blue: 0
                    }
                ]
            },
            Game {
                id: 2,
                hands: vec![
                    Hand {
                        red: 0,
                        green: 2,
                        blue: 1
                    },
                    Hand {
                        red: 1,
                        green: 3,
                        blue: 4
                    },
                    Hand {
                        red: 0,
                        green: 1,
                        blue: 1
                    }
                ]
            },
            Game {
                id: 3,
                hands: vec![
                    Hand {
                        red: 20,
                        green: 8,
                        blue: 6
                    },
                    Hand {
                        red: 4,
                        green: 13,
                        blue: 5
                    },
                    Hand {
                        red: 1,
                        green: 5,
                        blue: 0
                    }
                ]
            },
            Game {
                id: 4,
                hands: vec![
                    Hand {
                        red: 3,
                        green: 1,
                        blue: 6
                    },
                    Hand {
                        red: 6,
                        green: 3,
                        blue: 0
                    },
                    Hand {
                        red: 14,
                        green: 3,
                        blue: 15
                    }
                ]
            },
            Game {
                id: 5,
                hands: vec![
                    Hand {
                        red: 6,
                        green: 3,
                        blue: 1
                    },
                    Hand {
                        red: 1,
                        green: 2,
                        blue: 2
                    }
                ]
            }
        ],
        res.unwrap()
    );
}

#[test]
fn test_calculate_possible() {
    let input = vec![
        Game {
            id: 1,
            hands: vec![
                Hand {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                Hand {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Hand {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ],
        },
        Game {
            id: 2,
            hands: vec![
                Hand {
                    red: 0,
                    green: 2,
                    blue: 1,
                },
                Hand {
                    red: 1,
                    green: 3,
                    blue: 4,
                },
                Hand {
                    red: 0,
                    green: 1,
                    blue: 1,
                },
            ],
        },
        Game {
            id: 3,
            hands: vec![
                Hand {
                    red: 20,
                    green: 8,
                    blue: 6,
                },
                Hand {
                    red: 4,
                    green: 13,
                    blue: 5,
                },
                Hand {
                    red: 1,
                    green: 5,
                    blue: 0,
                },
            ],
        },
        Game {
            id: 4,
            hands: vec![
                Hand {
                    red: 3,
                    green: 1,
                    blue: 6,
                },
                Hand {
                    red: 6,
                    green: 3,
                    blue: 0,
                },
                Hand {
                    red: 14,
                    green: 3,
                    blue: 15,
                },
            ],
        },
        Game {
            id: 5,
            hands: vec![
                Hand {
                    red: 6,
                    green: 3,
                    blue: 1,
                },
                Hand {
                    red: 1,
                    green: 2,
                    blue: 2,
                },
            ],
        },
    ];

    let result = calculate_possible(&input, 12, 13, 14);

    assert_eq!(8, result);
}

#[test]
fn test_calculate_minimal() {
    let input = vec![
        Game {
            id: 1,
            hands: vec![
                Hand {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                Hand {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Hand {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ],
        },
        Game {
            id: 2,
            hands: vec![
                Hand {
                    red: 0,
                    green: 2,
                    blue: 1,
                },
                Hand {
                    red: 1,
                    green: 3,
                    blue: 4,
                },
                Hand {
                    red: 0,
                    green: 1,
                    blue: 1,
                },
            ],
        },
        Game {
            id: 3,
            hands: vec![
                Hand {
                    red: 20,
                    green: 8,
                    blue: 6,
                },
                Hand {
                    red: 4,
                    green: 13,
                    blue: 5,
                },
                Hand {
                    red: 1,
                    green: 5,
                    blue: 0,
                },
            ],
        },
        Game {
            id: 4,
            hands: vec![
                Hand {
                    red: 3,
                    green: 1,
                    blue: 6,
                },
                Hand {
                    red: 6,
                    green: 3,
                    blue: 0,
                },
                Hand {
                    red: 14,
                    green: 3,
                    blue: 15,
                },
            ],
        },
        Game {
            id: 5,
            hands: vec![
                Hand {
                    red: 6,
                    green: 3,
                    blue: 1,
                },
                Hand {
                    red: 1,
                    green: 2,
                    blue: 2,
                },
            ],
        },
    ];

    let result = calculate_minimal(&input);

    assert_eq!(2286, result);
}
