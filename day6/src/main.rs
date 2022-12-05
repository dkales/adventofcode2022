use color_eyre::Result;
use itertools::Itertools;
use std::{collections::HashSet, path::PathBuf};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day6", about = "Advent of Code Day 6")]
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
        .chars()
        .tuple_windows()
        .enumerate()
        .find(|(_idx, (a, b, c, d))| HashSet::from([a, b, c, d]).len() == 4)
        .unwrap()
        .0
        + 4)
}

fn solve_part2(input: &str) -> Result<usize> {
    Ok(input
        .chars()
        .collect::<Vec<_>>()
        .as_slice()
        .windows(14)
        .enumerate()
        .find(|(_idx, chars)| HashSet::from(<[char; 14]>::try_from(*chars).unwrap()).len() == 14)
        .unwrap()
        .0
        + 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kat() {
        let kats = vec![
            (r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#, (7, 19)),
            (r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#, (5, 23)),
            (r#"nppdvjthqldpwncqszvftbrmjlhg"#, (6, 23)),
            (r#"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"#, (10, 29)),
            (r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#, (11, 26)),
        ];

        for (input, output) in kats {
            assert_eq!(solve_part1(input).unwrap(), output.0);
            assert_eq!(solve_part2(input).unwrap(), output.1);
        }
    }
}
