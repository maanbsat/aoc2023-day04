use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{Context, Result};

const PATH: &str = "input.txt";

fn parse_num_sequence(seq: &str) -> HashSet<usize> {
    seq.split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part2(input: impl BufRead) -> Result<usize> {
    // num_cards holds number of times we need to run through each game card (0 indexed)
    let mut num_cards: Vec<usize> = Vec::new();
    // card_score is the 0-indexed score of each card
    let mut card_score: Vec<usize> = Vec::new();
    let mut total_seen: usize = 0;

    for l in input.lines() {
        let l = l?;
        total_seen += 1;
        let (first, second) = l.split_once(": ").unwrap();
        let game_number: usize = first
            .split(" ")
            .filter(|s| !s.is_empty())
            .last()
            .context("can't find game number")?
            .parse()?;
        // first time we're encountering this card
        if num_cards.len() < game_number {
            num_cards.push(0);
        }

        let (winning_numbers, my_numbers) =
            second.split_once(" | ").context("can't split numbers")?;
        let winning = parse_num_sequence(winning_numbers);
        let mine = parse_num_sequence(my_numbers);
        let intersection = winning.intersection(&mine).count();
        card_score.push(intersection);

        //println!("{:?}", num_cards);
        //println!("game {game_number}, intersection: {intersection}, total seen: {total_seen}");
        for i in game_number..(game_number + intersection) {
            //println!("i: {i}");
            if num_cards.len() <= i {
                num_cards.push(1);
            } else {
                num_cards[i] += 1;
            }
        }
    }

    // now we keep iterating while there are more cards to add
    loop {
        // find first non-zero index
        let next_iter = num_cards
            .iter()
            .enumerate()
            .filter(|(_, &i)| i > 0)
            .next()
            .and_then(|(i, &j)| Some((i, j)));
        if next_iter.is_none() {
            break;
        }
        let (game_index, how_many) = next_iter.unwrap();

        num_cards[game_index] = 0;
        total_seen += how_many;
        let score = card_score[game_index];
        //println!("game index: {game_index}, score: {score}, total_seen: {total_seen}");

        for i in (game_index + 1)..=(game_index + score) {
            num_cards[i] += how_many;
        }
        //println!("{:?}", num_cards);
    }

    Ok(total_seen)
}

fn main() -> Result<()> {
    let res = part2(io::BufReader::new(File::open(PATH)?))?;
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
            super::part2(io::BufReader::new(File::open("input_test.txt").unwrap())).unwrap(),
            30
        );
    }
}
