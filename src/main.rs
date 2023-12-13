use std::env;
use std::io::{self, BufRead};
use std::fs;

#[derive(Debug, Clone)]
struct ScratchCard {
    id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>
}

impl ScratchCard {
    fn winning_number_count(&self) -> u32 {
        self.numbers
            .iter()
            .map(|n| if self.winning_numbers.contains(n) { 1 } else { 0 })
            .sum::<u32>()
    }

    fn score(&self) -> usize {
        let base: usize = 2;
        let winning_number_count = self.winning_number_count();
        if winning_number_count == 0 { 0 } else { base.pow(winning_number_count - 1) }
    }
}


fn count_scratchcards(pile: &[ScratchCard]) -> usize {
    let mut counts: Vec<usize> = vec![1; pile.len()];
    for scratchcard in pile {
        let winning_number_count = scratchcard.winning_number_count();
        for i in scratchcard.id..(scratchcard.id + winning_number_count as usize){
            counts[i] += counts[scratchcard.id - 1];
        }
    }
    counts.into_iter().sum()
}


fn main() {
    let path = env::args().nth(1).expect("Missing required parameter path!");

    let data: Vec<ScratchCard> = io::BufReader::new(
        fs::File::open(path).expect("Could not open file!"))
        .lines()
        .map(|line| {
            let text = line.expect("Could not read line!");

            let (mut card_id, card_data) = text.split_once(": ").expect("Could not split id!");
            card_id = card_id.trim_start_matches("Card ").trim();

            let (winning_numbers, numbers) = card_data.split_once(" | ").expect("Could not split data!");
            
            ScratchCard { 
                id: card_id.parse::<usize>().expect("Invalid ID!"), 
                winning_numbers: winning_numbers
                    .split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect(),
                numbers: numbers
                    .split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect()
            }
        })
        .collect();

    println!("Total scratchcards: {}", count_scratchcards(&data));
}
