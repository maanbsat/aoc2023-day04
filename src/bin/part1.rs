use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::Result;

const PATH: &str = "input.txt";

fn parse_num_sequence(seq: &str) -> HashSet<usize> {
    seq.split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1(input: impl BufRead) -> Result<usize> {
    let base: usize = 2;

    Ok(input
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let (winning_numbers, my_numbers) =
                l.split_once(": ").unwrap().1.split_once(" | ").unwrap();
            let winning = parse_num_sequence(winning_numbers);
            let mine = parse_num_sequence(my_numbers);
            let intersection = winning.intersection(&mine).count();
            if intersection == 0 || intersection == 1 {
                intersection
            } else {
                base.pow((intersection as u32) - 1)
            }
        })
        .sum())
}

fn main() -> Result<()> {
    let res = part1(io::BufReader::new(File::open(PATH)?))?;
    println!("{res}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io;

    #[test]
    fn t1() {
        assert_eq!(
            super::part1(io::BufReader::new(File::open("input_test.txt").unwrap())).unwrap(),
            13
        );
    }
}
