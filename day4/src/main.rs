use color_eyre::Result;
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day4", about = "Advent of Code Day 4")]
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
        .map(|x| x.split_once(',').unwrap())
        .map(|(a, b)| (a.split_once('-').unwrap(), b.split_once('-').unwrap()))
        .map(|((a, b), (x, y))| {
            (
                a.parse::<usize>().unwrap()..=b.parse::<usize>().unwrap(),
                x.parse::<usize>().unwrap()..=y.parse::<usize>().unwrap(),
            )
        })
        .filter(|(r, s)| {
            (r.contains(s.start()) && r.contains(s.end()))
                || (s.contains(r.start()) && s.contains(r.end()))
        })
        .count())
}

fn solve_part2(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|x| x.split_once(',').unwrap())
        .map(|(a, b)| (a.split_once('-').unwrap(), b.split_once('-').unwrap()))
        .map(|((a, b), (x, y))| {
            (
                a.parse::<usize>().unwrap()..=b.parse::<usize>().unwrap(),
                x.parse::<usize>().unwrap()..=y.parse::<usize>().unwrap(),
            )
        })
        .filter(|(r, s)| {
            (r.contains(s.start()) || r.contains(s.end()))
                || (s.contains(r.start()) || s.contains(r.end()))
        })
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kat() {
        let kat = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
        assert_eq!(solve_part1(kat).unwrap(), 2);
        assert_eq!(solve_part2(kat).unwrap(), 4);
    }
}
