use color_eyre::{eyre::eyre, Report, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, not_line_ending, space1},
    combinator::{map, map_res, opt, value},
    multi::many0,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use std::{path::PathBuf, str::FromStr};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day7", about = "Advent of Code Day 7")]
struct Args {
    #[structopt(parse(from_os_str), default_value = "./input")]
    input_file: PathBuf,
}

#[derive(Debug)]
struct Dir {
    _name: String,
    dirs: Vec<Dir>,
    files: Vec<(String, usize)>,
}

fn parse_ls(input: &str) -> IResult<&str, Vec<(String, usize)>> {
    // skip $ ls
    let (input, _) = terminated(tag("$ ls"), line_ending)(input)?;
    map(
        many0(alt((
            value(
                None,
                terminated(preceded(tag("dir "), not_line_ending), line_ending),
            ),
            map(
                terminated(
                    separated_pair(
                        map_res(digit1, str::parse::<usize>),
                        space1,
                        not_line_ending,
                    ),
                    line_ending,
                ),
                Some,
            ),
        ))),
        |x| {
            x.into_iter()
                .flatten()
                .map(|(size, name)| (name.to_owned(), size))
                .collect()
        },
    )(input)
}

fn parse_dir(input: &str) -> IResult<&str, Dir> {
    // $ cd <dirname>
    let (input, name) = terminated(preceded(tag("$ cd "), not_line_ending), line_ending)(input)?;
    let (input, files) = parse_ls(input)?;
    let (input, dirs) = many0(parse_dir)(input)?;
    let (input, _) = opt(terminated(tag("$ cd .."), line_ending))(input)?;

    Ok((
        input,
        Dir {
            _name: name.to_owned(),
            dirs,
            files,
        },
    ))
}

impl FromStr for Dir {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rest, dir) = parse_dir(s).map_err(|e| e.to_owned())?;

        if !rest.is_empty() {
            return Err(eyre!("Have stuff left: {}", rest));
        }
        Ok(dir)
    }
}

impl Dir {
    fn solve_part1(&self) -> usize {
        let local_size = self.size();

        let dirs_sizes = self.dirs.iter().map(|d| d.solve_part1()).sum();

        if local_size <= 100000 {
            local_size + dirs_sizes
        } else {
            dirs_sizes
        }
    }
    fn size(&self) -> usize {
        self.files.iter().map(|x| x.1).sum::<usize>()
            + self.dirs.iter().map(|d| d.size()).sum::<usize>()
    }
    fn find_smallest_subdir_above(&self, target: usize) -> Option<usize> {
        let subdirs = self
            .dirs
            .iter()
            .filter_map(|x| x.find_smallest_subdir_above(target))
            .min();

        match (subdirs, self.size() > target) {
            (None, true) => Some(self.size()),
            (None, false) => None,
            (Some(x), true) => Some(x),
            _ => unreachable!(),
        }
    }
    fn solve_part2(&self) -> usize {
        let local_size = self.size();
        let occupied = 70000000 - local_size;
        let needed = 30000000 - occupied;

        self.find_smallest_subdir_above(needed).unwrap()
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let input = std::fs::read_to_string(args.input_file)?;

    let dir = Dir::from_str(&input)?;
    println!("Solution to part1: {}", dir.solve_part1());
    println!("Solution to part2: {}", dir.solve_part2());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kat() {
        let kat = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;

        let dir = Dir::from_str(kat).unwrap();

        assert_eq!(dir.solve_part1(), 95437);
        assert_eq!(dir.solve_part2(), 24933642);
    }
}
