use rust::{common::puzzle_input_reader::read_file, day1};

fn main() {
    let mut input1_1 = read_file("puzzles/day1-1.txt");
    let mut input1_2 = read_file("puzzles/day1-1.txt");
    println!("Day1-1: {}", day1::run_part_1(&mut input1_1));
    println!("Day1-2: {}", day1::run_part_2(&mut input1_2));
}
