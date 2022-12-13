use color_eyre::Result;
use std::{path::PathBuf, vec};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day10", about = "Advent of Code Day 10")]
struct Args {
    #[structopt(parse(from_os_str), default_value = "./input")]
    input_file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let input = std::fs::read_to_string(args.input_file)?;
    let input = input.trim_end();

    println!("Solution to part1: {}", solve_part1(input)?);
    println!("Solution to part2: \n{}", solve_part2(input)?);

    Ok(())
}

fn solve_part1(input: &str) -> Result<isize> {
    let mut values = vec![1isize];
    for line in input.lines() {
        values.push(*values.last().unwrap());
        if line == "noop" {
            continue;
        }
        let (_, b) = line.split_once(' ').unwrap();
        let val = b.parse::<isize>().unwrap();
        values.push(*values.last().unwrap() + val);
    }

    Ok(values
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(idx, val)| (idx + 1) as isize * *val)
        .sum())
}

fn solve_part2(input: &str) -> Result<String> {
    let mut values = vec![1isize];
    for line in input.lines() {
        values.push(*values.last().unwrap());
        if line == "noop" {
            continue;
        }
        let (_, b) = line.split_once(' ').unwrap();
        let val = b.parse::<isize>().unwrap();
        values.push(*values.last().unwrap() + val);
    }
    let mut output = String::new();

    for (idx, val) in values.iter().enumerate().take(240) {
        output += if [val - 1, *val, val + 1].contains(&(idx as isize % 40)) {
            "#"
        } else {
            " "
        };
        if idx % 40 == 39 && idx != 239 {
            output += "\n";
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kat() {
        let kat = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

        assert_eq!(solve_part1(kat).unwrap(), 13140);
        assert_eq!(
            solve_part2(kat).unwrap(),
            r#"##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     "#
        );
    }
}
