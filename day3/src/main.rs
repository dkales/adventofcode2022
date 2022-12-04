use color_eyre::Result;
use std::{collections::HashSet, path::PathBuf};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day3", about = "Advent of Code Day 3")]
struct Args {
    #[structopt(parse(from_os_str), default_value = "./input")]
    input_file: PathBuf,
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
    Ok(input
        .lines()
        .map(|x| {
            let len = x.len();
            let x1: HashSet<_> = x[..len / 2].chars().collect();
            x[len / 2..].chars().find(|x| x1.contains(x)).unwrap()
        })
        .map(|x| {
            if x.is_ascii_lowercase() {
                x as usize - 'a' as usize + 1
            } else {
                x as usize - 'A' as usize + 27
            }
        })
        .sum())
}

fn solve_part2(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .collect::<Vec<_>>()
        .as_slice()
        .chunks_exact(3)
        .map(|a| {
            let x1: HashSet<_> = a[0].chars().collect();
            let x2: HashSet<_> = a[1].chars().filter(|x| x1.contains(x)).collect();
            a[2].chars().find(|x| x2.contains(x)).unwrap()
        })
        .map(|x| {
            if x.is_ascii_lowercase() {
                x as usize - 'a' as usize + 1
            } else {
                x as usize - 'A' as usize + 27
            }
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kat() {
        let kat = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(solve_part1(kat).unwrap(), 157);
        assert_eq!(solve_part2(kat).unwrap(), 70);
    }
}
