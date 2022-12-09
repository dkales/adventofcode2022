use color_eyre::Result;
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day8", about = "Advent of Code Day 8")]
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
    let trees: Vec<Vec<_>> = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    let (width, height) = (trees[0].len(), trees.len());

    let mut visible = 0;
    for x in 0..width {
        for y in 0..height {
            if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                visible += 1;
                continue;
            }
            // naive approach, no dynamic programming
            // check all 4 directions

            let my_val = trees[x][y];
            // above
            if (0..y).map(|cur_y| trees[x][cur_y]).all(|h| h < my_val) {
                visible += 1;
                continue;
            }
            // below
            if (y + 1..height)
                .map(|cur_y| trees[x][cur_y])
                .all(|h| h < my_val)
            {
                visible += 1;
                continue;
            }
            // left
            if (0..x).map(|cur_x| trees[cur_x][y]).all(|h| h < my_val) {
                visible += 1;
                continue;
            }
            // right
            if (x + 1..width)
                .map(|cur_x| trees[cur_x][y])
                .all(|h| h < my_val)
            {
                visible += 1;
                continue;
            }
        }
    }
    Ok(visible)
}

fn solve_part2(input: &str) -> Result<usize> {
    let trees: Vec<Vec<_>> = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    let (width, height) = (trees[0].len(), trees.len());

    let mut best_score = 0;
    for x in 0..width {
        for y in 0..height {
            // these have a score of 0
            if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                continue;
            }

            let my_val = trees[x][y];
            // above
            let mut above_score = (0..y)
                .rev()
                .map(|cur_y| trees[x][cur_y])
                .take_while(|&h| h < my_val)
                .count();
            if !(0..y).map(|cur_y| trees[x][cur_y]).all(|h| h < my_val) {
                above_score += 1;
            }
            // below
            let mut below_score = (y + 1..height)
                .map(|cur_y| trees[x][cur_y])
                .take_while(|&h| h < my_val)
                .count();
            if !(y + 1..height)
                .map(|cur_y| trees[x][cur_y])
                .all(|h| h < my_val)
            {
                below_score += 1;
            }
            // left
            let mut left_score = (0..x)
                .rev()
                .map(|cur_x| trees[cur_x][y])
                .take_while(|&h| h < my_val)
                .count();
            if !(0..x).map(|cur_x| trees[cur_x][y]).all(|h| h < my_val) {
                left_score += 1;
            }
            // right
            let mut right_score = (x + 1..width)
                .map(|cur_x| trees[cur_x][y])
                .take_while(|&h| h < my_val)
                .count();
            if !(x + 1..width)
                .map(|cur_x| trees[cur_x][y])
                .all(|h| h < my_val)
            {
                right_score += 1;
            }
            let score = above_score * below_score * right_score * left_score;
            if score > best_score {
                best_score = score;
            }
        }
    }
    Ok(best_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kat() {
        let kat = r#"30373
25512
65332
33549
35390"#;

        assert_eq!(solve_part1(kat).unwrap(), 21);
        assert_eq!(solve_part2(kat).unwrap(), 8);
    }
}
