use color_eyre::{Report, Result};
use regex::Regex;
use std::{collections::VecDeque, path::PathBuf, str::FromStr};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day5", about = "Advent of Code Day 5")]
struct Args {
    #[structopt(parse(from_os_str), default_value = "./input")]
    input_file: PathBuf,
}

#[derive(Debug, Clone)]
struct Puzzle {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

impl FromStr for Puzzle {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack_lines = Vec::new();
        let mut stack_done = false;
        let mut stacks: Vec<VecDeque<char>> = Vec::new();
        let mut moves: Vec<Move> = Vec::new();
        for line in s.lines() {
            if line.is_empty() {
                continue;
            }
            if !line.starts_with(" 1 ") && !stack_done {
                stack_lines.push(line);
                continue;
            }
            if !stack_done {
                stack_done = true;
                let num_stacks: usize = line.split_ascii_whitespace().last().unwrap().parse()?;
                stacks.resize(num_stacks, Default::default());

                for sl in stack_lines.iter() {
                    for (idx, c) in sl.chars().skip(1).step_by(4).enumerate() {
                        if c.is_alphabetic() {
                            stacks[idx].push_front(c);
                        }
                    }
                }
                continue;
            }
            moves.push(line.parse()?);
        }

        Ok(Puzzle { stacks, moves })
    }
}

impl Puzzle {
    fn solve_part1(mut self) -> String {
        for m in self.moves {
            for _ in 0..m.amount {
                let c = self.stacks[m.from].pop_back().unwrap();
                self.stacks[m.to].push_back(c);
            }
        }

        self.stacks
            .iter_mut()
            .map(|x| x.pop_back().unwrap())
            .collect()
    }
    fn solve_part2(mut self) -> String {
        for m in self.moves {
            let mut buf = Vec::with_capacity(m.amount);
            for _ in 0..m.amount {
                buf.push(self.stacks[m.from].pop_back().unwrap());
            }
            self.stacks[m.to].extend(buf.iter().rev());
        }

        self.stacks
            .iter_mut()
            .map(|x| x.pop_back().unwrap())
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r#"move (\d+) from (\d+) to (\d+)"#)?;

        let cap = regex
            .captures_iter(s)
            .next()
            .ok_or(color_eyre::eyre::eyre!("invalid move"))?;

        Ok(Move {
            amount: cap[1].parse()?,
            from: cap[2].parse::<usize>()? - 1,
            to: cap[3].parse::<usize>()? - 1,
        })
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let input = std::fs::read_to_string(args.input_file)?;
    let input = input.trim_end();

    let puz: Puzzle = input.parse()?;

    println!("Solution to part1: {}", puz.clone().solve_part1());
    println!("Solution to part2: {}", puz.solve_part2());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kat() {
        let kat = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
        let puz: Puzzle = kat.parse().unwrap();
        assert_eq!(puz.clone().solve_part1(), String::from("CMZ"));
        assert_eq!(puz.solve_part2(), String::from("MCD"));
    }
}
