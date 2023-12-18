use crate::common::puzzle_input_reader::PuzzleInput;
use std::fs::File;

struct CalibrationValue {
    pub value: u64,
}

impl From<String> for CalibrationValue {
    fn from(value: String) -> Self {
        let collection = value
            .as_bytes()
            .to_owned()
            .into_iter()
            .filter(|&b| b >= b'0' && b <= b'9')
            .collect::<Vec<u8>>();
        let new_value = String::from_utf8(vec![collection[0], collection[collection.len() - 1]])
            .unwrap()
            .parse::<u64>()
            .unwrap();
        CalibrationValue { value: new_value }
    }
}

struct UpdatedCalibrationValue {
    pub value: u64,
}

#[derive(Clone, Copy)]
enum Search<'a> {
    Number(&'a str),
    Word(&'a str, &'a str),
}

impl<'a> Search<'a> {
    fn value(&self) -> &'a str {
        match self {
            Self::Number(s) => s,
            Self::Word(s, _) => s,
        }
    }
    fn as_num(&self) -> u8 {
        match self {
            Self::Number(s) => s.as_bytes()[0],
            Self::Word(_, s) => s.as_bytes()[0],
        }
    }
}

const PART_2_SEARCH: [Search<'static>; 18] = [
    Search::Number("1"),
    Search::Number("2"),
    Search::Number("3"),
    Search::Number("4"),
    Search::Number("5"),
    Search::Number("6"),
    Search::Number("7"),
    Search::Number("8"),
    Search::Number("9"),
    Search::Word("one", "1"),
    Search::Word("two", "2"),
    Search::Word("three", "3"),
    Search::Word("four", "4"),
    Search::Word("five", "5"),
    Search::Word("six", "6"),
    Search::Word("seven", "7"),
    Search::Word("eight", "8"),
    Search::Word("nine", "9"),
];

fn find_first_last<'a, 'b>(
    search_string: &'a str,
    options: Vec<Search<'b>>,
) -> (Search<'b>, Search<'b>) {
    (
        options
            .iter()
            .map(|option| (search_string.find(option.value()), option))
            .filter(|result| result.0.is_some())
            .min_by(|x, y| x.0.unwrap().cmp(&y.0.unwrap()))
            .unwrap()
            .1
            .clone(),
        options
            .iter()
            .map(|option| (search_string.rfind(option.value()), option))
            .filter(|result| result.0.is_some())
            .max_by(|x, y| x.0.unwrap().cmp(&y.0.unwrap()))
            .unwrap()
            .1
            .clone(),
    )
}

impl From<String> for UpdatedCalibrationValue {
    fn from(value: String) -> Self {
        // print!("old: {:?}, ", value);
        let (first, last) = find_first_last(&value, Vec::from(PART_2_SEARCH));
        let new_value = String::from_utf8(vec![first.as_num(), last.as_num()])
            .unwrap()
            .parse::<u64>()
            .unwrap();
        // println!("Final: {:?}", new_value);
        UpdatedCalibrationValue { value: new_value }
    }
}

pub fn run_part_1(input: &mut File) -> u64 {
    let puzzle_input: PuzzleInput<CalibrationValue> = PuzzleInput::from_file(input).unwrap();
    puzzle_input.into_iter().map(|c| c.value).sum()
}

pub fn run_part_2(input: &mut File) -> u64 {
    let puzzle_input: PuzzleInput<UpdatedCalibrationValue> = PuzzleInput::from_file(input).unwrap();
    // for input in puzzle_input.into_iter() {
    //     println!("Value: {}", input.value)
    // }
    puzzle_input.into_iter().map(|c| c.value).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::puzzle_input_reader::test::MockFile;
    #[test]
    fn test_simple_input_part_1() {
        let mut input = MockFile::with_contents("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");
        let result = run_part_1(&mut input.get_file());
        let expected_result = 142;
        assert_eq!(expected_result, result);
    }
    #[test]

    fn test_simple_input_part_2() {
        let mut input = MockFile::with_contents("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen");
        let result = run_part_2(&mut input.get_file());
        let expected_result = 281;
        assert_eq!(expected_result, result);
    }
}
