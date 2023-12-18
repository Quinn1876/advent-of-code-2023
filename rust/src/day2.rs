use core::panic;
use std::fs::File;

use regex::Regex;

use crate::common::puzzle_input_reader::PuzzleInput;

pub struct Hand {
    num_green: u32,
    num_red: u32,
    num_blue: u32,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            num_blue: 0,
            num_green: 0,
            num_red: 0,
        }
    }
}

pub struct Game {
    id: usize,
    hands: Vec<Hand>,
}

impl From<String> for Game {
    fn from(value: String) -> Self {
        let re_game_id = Regex::new(r"Game ([0-9]*)").unwrap();
        let re_entry = Regex::new(r"([0-9]*) (blue|green|red)").unwrap();
        let values = value.split(": ").collect::<Vec<&str>>();
        let game_id: &str = &re_game_id.captures(values[0]).unwrap()[1];

        let hands = values[1].split(";");

        let mut result_hands: Vec<Hand> = Vec::new();

        for hand in hands {
            let mut new_hand = Hand::new();
            for entry in hand.split(",") {
                let Some(entry_capture) = re_entry.captures(entry) else {
                    break;
                };
                match &entry_capture[2] {
                    "blue" => new_hand.num_blue = entry_capture[1].parse::<u32>().unwrap(),
                    "red" => new_hand.num_red = entry_capture[1].parse::<u32>().unwrap(),
                    "green" => new_hand.num_green = entry_capture[1].parse::<u32>().unwrap(),
                    _ => panic!("This should not hit"),
                }
            }
            result_hands.push(new_hand);
        }

        Self {
            id: String::from(game_id).parse::<usize>().unwrap(),
            hands: result_hands,
        }
    }
}

const NUM_RED: u32 = 12;
const NUM_GREEN: u32 = 13;
const NUM_BLUE: u32 = 14;

pub fn run_part_1(input: &mut File) -> usize {
    let puzzle_input: PuzzleInput<Game> = PuzzleInput::from_file(input).unwrap();
    puzzle_input
        .into_iter()
        .filter(|&game| {
            game.hands.iter().all(|hand| {
                hand.num_blue <= NUM_BLUE && hand.num_green <= NUM_GREEN && hand.num_red <= NUM_RED
            })
        })
        .map(|game| game.id)
        .sum()
}

pub fn run_part_2(input: &mut File) -> u32 {
    let puzzle_input: PuzzleInput<Game> = PuzzleInput::from_file(input).unwrap();
    puzzle_input
        .into_iter()
        .map(|game| {
            let min_red = game
                .hands
                .iter()
                .max_by(|&x, &y| x.num_red.cmp(&y.num_red))
                .unwrap()
                .num_red;
            let min_green = game
                .hands
                .iter()
                .max_by(|&x, &y| x.num_green.cmp(&y.num_green))
                .unwrap()
                .num_green;
            let min_blue = game
                .hands
                .iter()
                .max_by(|&x, &y| x.num_blue.cmp(&y.num_blue))
                .unwrap()
                .num_blue;
            min_red * min_blue * min_green
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::common::puzzle_input_reader::test::MockFile;

    use super::{run_part_1, run_part_2, Game};

    #[test]
    fn test_input_parsing() {
        let game = Game::from(String::from(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        ));
        assert_eq!(game.hands[0].num_blue, 3);
        assert_eq!(game.hands[0].num_red, 4);
        assert_eq!(game.hands[0].num_green, 0);
        assert_eq!(game.hands[1].num_blue, 6);
        assert_eq!(game.hands[1].num_red, 1);
        assert_eq!(game.hands[1].num_green, 2);
        assert_eq!(game.hands[2].num_blue, 0);
        assert_eq!(game.hands[2].num_red, 0);
        assert_eq!(game.hands[2].num_green, 2);
        assert_eq!(game.id, 1);
    }

    #[test]
    fn test_sample_input_1() {
        let mut sample_input = MockFile::with_contents("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        let actual_result = run_part_1(sample_input.get_file());
        let expected_result = 8;
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_sample_input_2() {
        let mut sample_input = MockFile::with_contents("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        let actual_result = run_part_2(sample_input.get_file());
        let expected_result = 2286;
        assert_eq!(actual_result, expected_result);
    }
}
