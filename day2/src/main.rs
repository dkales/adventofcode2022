use std::{iter::Sum, path::PathBuf, str::FromStr};

use color_eyre::{Report, Result};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day2", about = "Advent of Code Day 2")]
struct Args {
    #[structopt(parse(from_os_str), default_value = "./input")]
    input_file: PathBuf,
}

struct RoundPart1 {
    score: usize,
}

impl FromStr for RoundPart1 {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split_ascii_whitespace();
        let f = i.next().ok_or(color_eyre::eyre::eyre!("invalid input"))?;
        let x = i.next().ok_or(color_eyre::eyre::eyre!("invalid input"))?;
        if i.next().is_some() {
            return Err(color_eyre::eyre::eyre!("invalid input"));
        }
        let mut score = 0;
        match x {
            "X" => {
                score += 1;
                match f {
                    "A" => score += 3,
                    "B" => score += 0,
                    "C" => score += 6,
                    _ => return Err(color_eyre::eyre::eyre!("invalid input")),
                }
            }
            "Y" => {
                score += 2;
                match f {
                    "A" => score += 6,
                    "B" => score += 3,
                    "C" => score += 0,
                    _ => return Err(color_eyre::eyre::eyre!("invalid input")),
                }
            }
            "Z" => {
                score += 3;
                match f {
                    "A" => score += 0,
                    "B" => score += 6,
                    "C" => score += 3,
                    _ => return Err(color_eyre::eyre::eyre!("invalid input")),
                }
            }
            _ => return Err(color_eyre::eyre::eyre!("invalid input")),
        }

        Ok(RoundPart1 { score })
    }
}
impl Sum<RoundPart1> for usize {
    fn sum<I: Iterator<Item = RoundPart1>>(iter: I) -> Self {
        iter.map(|x| x.score).sum()
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let input = std::fs::read_to_string(args.input_file)?;
    let input = input.trim_end();

    println!("Solution to part1: {}", solve_part1(input)?);
    println!("Solution to part2: {}", solve_part2(input)?);

    Ok(())
}

fn solve_part1(input: &str) -> Result<usize> {
    input.lines().map(RoundPart1::from_str).sum()
}

struct RoundPart2 {
    score: usize,
}

impl FromStr for RoundPart2 {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split_ascii_whitespace();
        let f = i.next().ok_or(color_eyre::eyre::eyre!("invalid input"))?;
        let x = i.next().ok_or(color_eyre::eyre::eyre!("invalid input"))?;
        if i.next().is_some() {
            return Err(color_eyre::eyre::eyre!("invalid input"));
        }
        let mut score = 0;
        match x {
            "X" => match f {
                "A" => score += 3,
                "B" => score += 1,
                "C" => score += 2,
                _ => return Err(color_eyre::eyre::eyre!("invalid input")),
            },
            "Y" => match f {
                "A" => score += 1 + 3,
                "B" => score += 2 + 3,
                "C" => score += 3 + 3,
                _ => return Err(color_eyre::eyre::eyre!("invalid input")),
            },
            "Z" => match f {
                "A" => score += 2 + 6,
                "B" => score += 3 + 6,
                "C" => score += 1 + 6,
                _ => return Err(color_eyre::eyre::eyre!("invalid input")),
            },
            _ => return Err(color_eyre::eyre::eyre!("invalid input")),
        }

        Ok(RoundPart2 { score })
    }
}
impl Sum<RoundPart2> for usize {
    fn sum<I: Iterator<Item = RoundPart2>>(iter: I) -> Self {
        iter.map(|x| x.score).sum()
    }
}

fn solve_part2(input: &str) -> Result<usize> {
    input.lines().map(RoundPart2::from_str).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kat() {
        let kat = r#"A Y
B X
C Z"#;
        assert_eq!(solve_part1(kat).unwrap(), 15);
        assert_eq!(solve_part2(kat).unwrap(), 12);
    }
}
