use std::path::PathBuf;

use color_eyre::Result;
use itertools::Itertools;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day1", about = "Advent of Code Day 1")]
struct Args {
    #[structopt(parse(from_os_str), default_value = "./input")]
    input_file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let input = std::fs::read_to_string(args.input_file)?;

    println!("Solution to part1: {}", solve_part1(&input)?);
    println!("Solution to part2: {}", solve_part2(&input)?);

    Ok(())
}

fn solve_part1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(str::parse::<usize>)
        .collect::<Vec<_>>()
        .split(|x| x.is_err())
        .map(|x| x.iter().map(|x| x.as_ref().unwrap()).sum())
        .max()
        .unwrap())
}

fn solve_part2(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(str::parse::<usize>)
        .collect::<Vec<_>>()
        .split(|x| x.is_err())
        .map(|x| x.iter().map(|x| x.as_ref().unwrap()).sum::<usize>())
        .sorted()
        .rev()
        .take(3)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kat() {
        let kat = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
        assert_eq!(solve_part1(kat).unwrap(), 24000);
        assert_eq!(solve_part2(kat).unwrap(), 45000);
    }
}
