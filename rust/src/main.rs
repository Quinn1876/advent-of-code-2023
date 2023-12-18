use rust::{common::puzzle_input_reader::read_file, day1, day2, day3};

fn main() {
    let mut input1_1 = read_file("puzzles/day1-1.txt");
    let mut input1_2 = read_file("puzzles/day1-1.txt");
    println!("Day1-1: {}", day1::run_part_1(&mut input1_1));
    println!("Day1-2: {}", day1::run_part_2(&mut input1_2));

    let mut input2_1 = read_file("puzzles/day2.txt");
    let mut input2_2 = read_file("puzzles/day2.txt");
    println!("Day2-1: {}", day2::run_part_1(&mut input2_1));
    println!("Day2-2: {}", day2::run_part_2(&mut input2_2));
    let mut input3_1 = read_file("puzzles/day3.txt");
    let mut input3_2 = read_file("puzzles/day3.txt");
    println!("Day3-1: {}", day3::run_part_1(&mut input3_1));
    println!("Day3-2: {}", day3::run_part_2(&mut input3_2));
}
