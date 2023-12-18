use std::{collections::{HashMap, hash_set, HashSet}, fs::File};

use crate::common::puzzle_input_reader::PuzzleInput;

enum SchematicEntryBuilder {
    Number { value: u32, width: u32 },
    BlankSpace { width: u32 },
    Symbol,
    Gear,
}

struct SchematicEntryBuilderLine {
    line: Vec<SchematicEntryBuilder>,
}

impl From<String> for SchematicEntryBuilderLine {
    fn from(value: String) -> Self {
        let mut current_state = SchematicParserState::from(value.as_bytes()[0]);
        let mut current_value = Vec::<u8>::new();
        let mut line: Vec<SchematicEntryBuilder> = Vec::new();
        for &character in value.as_bytes().iter() {
            let next_state = SchematicParserState::from(character);
            if current_state != next_state {
                match current_state {
                    SchematicParserState::Number => {
                        // println!("current value {:?}", current_value);
                        // println!("Current Value as string: {:?}", String::from_utf8(current_value.clone()));
                        line.push(SchematicEntryBuilder::Number {
                            value: String::from_utf8(current_value.clone())
                                .unwrap()
                                .parse()
                                .unwrap(),
                            width: current_value.len() as u32,
                        });
                    }
                    SchematicParserState::Dot => {
                        line.push(SchematicEntryBuilder::BlankSpace {
                            width: current_value.len() as u32,
                        });
                    }
                    SchematicParserState::Symbol => {
                        for _ in 0..current_value.len() {
                            line.push(SchematicEntryBuilder::Symbol);
                        }
                    },
                    SchematicParserState::Gear => {
                        for _ in 0..current_value.len() {
                            line.push(SchematicEntryBuilder::Gear);
                        }
                    }
                }
                current_value.clear();
            }
            current_value.push(character);
            current_state = next_state;
        }
        match current_state {
            SchematicParserState::Number => {
                line.push(SchematicEntryBuilder::Number {
                    value: String::from_utf8(current_value.clone())
                        .unwrap()
                        .parse()
                        .unwrap(),
                    width: current_value.len() as u32,
                });
            }
            SchematicParserState::Dot => {
                line.push(SchematicEntryBuilder::BlankSpace {
                    width: current_value.len() as u32,
                });
            }
            SchematicParserState::Symbol => {
                for _ in 0..current_value.len() {
                    line.push(SchematicEntryBuilder::Symbol);
                }
            }
            SchematicParserState::Gear => {
                for _ in 0..current_value.len() {
                    line.push(SchematicEntryBuilder::Gear);
                }
            }
        }
        Self { line }
    }
}

#[derive(PartialEq)]
enum SchematicParserState {
    Number,
    Dot,
    Symbol,
    Gear
}

fn is_numeric(byte: u8) -> bool {
    assert!(b'0' < b'9');
    byte >= b'0' && byte <= b'9'
}

impl From<u8> for SchematicParserState {
    fn from(value: u8) -> Self {
        if is_numeric(value) {
            SchematicParserState::Number
        } else if value == b'.' {
            SchematicParserState::Dot
        } else if value == b'*' {
            SchematicParserState::Gear
        }
        else {
            SchematicParserState::Symbol
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct SchematicPosition {
    row: isize,
    col: isize,
}

impl SchematicPosition {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone, Copy)]
enum SchematicEntry {
    Number {
        value: u32,
        first_pos: SchematicPosition,
        last_pos: SchematicPosition,
    },
    #[allow(dead_code)]
    Symbol{ pos: SchematicPosition },
    #[allow(dead_code)]
    Gear{ pos: SchematicPosition },
}

enum SearchBoxState {
    Top,
    Right,
    Bottom,
    Left,
    Finished
}

struct SchematicSearchBox {
    top_left: SchematicPosition,
    bottom_right: SchematicPosition,
    current_position: SchematicPosition,
    state: SearchBoxState
}

impl Iterator for SchematicSearchBox {
    type Item = SchematicPosition;
    fn next(&mut self) -> Option<Self::Item> {
        let next_state: SearchBoxState;
        let result = self.current_position.clone();
        match self.state {
            SearchBoxState::Finished => return None,
            SearchBoxState::Top => {
                self.current_position.col += 1;
                if self.current_position.col == self.bottom_right.col {
                    next_state = SearchBoxState::Right;
                } else {
                    next_state = SearchBoxState::Top;
                }
            },
            SearchBoxState::Right => {
                self.current_position.row += 1;
                if self.current_position.row == self.bottom_right.row {
                    next_state = SearchBoxState::Bottom;
                } else {
                    next_state = SearchBoxState::Right;
                }
            },
            SearchBoxState::Bottom => {
                self.current_position.col -= 1;
                if self.current_position.col == self.top_left.col {
                    next_state = SearchBoxState::Left;
                } else {
                    next_state = SearchBoxState::Bottom;
                }
            },
            SearchBoxState::Left => {
                self.current_position.row -= 1;
                if self.current_position.row == self.top_left.row {
                    next_state = SearchBoxState::Finished;
                } else {
                    next_state = SearchBoxState::Left;
                }
            }
        }
        self.state = next_state;
        Some(result)
    }
}

impl<'a> IntoIterator for &'a SchematicEntry {
    type Item = SchematicPosition;
    type IntoIter = SchematicSearchBox;

    fn into_iter(self) -> Self::IntoIter {
        match *self {
            SchematicEntry::Number { value:_, first_pos, last_pos } => {
                let mut top_left = first_pos.clone();
                top_left.row -= 1;
                top_left.col -= 1;
                let mut bottom_right = last_pos.clone();
                bottom_right.row += 1;
                bottom_right.col += 1;
                Self::IntoIter {
                    top_left,
                    bottom_right,
                    current_position: top_left.clone(),
                    state: SearchBoxState::Top,
                }
            },
            SchematicEntry::Gear { pos } => {
                let mut top_left = pos.clone();
                top_left.row -= 1;
                top_left.col -= 1;
                let mut bottom_right = pos.clone();
                bottom_right.row += 1;
                bottom_right.col += 1;
                Self::IntoIter {
                    top_left,
                    bottom_right,
                    current_position: top_left.clone(),
                    state: SearchBoxState::Top,
                }
            }
            _ => panic!("Only call on number or gear")
        }
    }
}

struct Schematic {
    schematic_map: HashMap<SchematicPosition, SchematicEntry>,
}

impl From<PuzzleInput<SchematicEntryBuilderLine>> for Schematic {
    fn from(value: PuzzleInput<SchematicEntryBuilderLine>) -> Self {
        let mut schematic_map: HashMap<SchematicPosition, SchematicEntry> = HashMap::new();
        for (row, line) in value.into_iter().enumerate() {
            let mut current_col: isize = 0;
            for builder in line.line.iter() {
                match *builder {
                    SchematicEntryBuilder::BlankSpace { width } => {
                        current_col += width as isize;
                    }
                    SchematicEntryBuilder::Number { value, width } => {
                        let entry = SchematicEntry::Number {
                            value,
                            first_pos: SchematicPosition::new(row as isize, current_col),
                            last_pos: SchematicPosition::new(
                                row as isize,
                                current_col + (width as isize) - 1,
                            ),
                        };
                        for c in 0..width {
                            assert!(schematic_map.insert(
                                SchematicPosition::new(row as isize, current_col + (c as isize)),
                                entry.clone(),
                            ).is_none());
                        }
                        current_col += width as isize;
                    }
                    SchematicEntryBuilder::Symbol => {
                        assert!(schematic_map.insert(
                            SchematicPosition::new(row as isize, current_col),
                            SchematicEntry::Symbol{ pos: SchematicPosition::new(row as isize, current_col) },
                        ).is_none());
                        current_col += 1;
                    },
                    SchematicEntryBuilder::Gear => {
                        assert!(schematic_map.insert(
                            SchematicPosition::new(row as isize, current_col),
                            SchematicEntry::Gear{ pos: SchematicPosition::new(row as isize, current_col) },
                        ).is_none());
                        current_col += 1;
                    }
                }
            }
        }
        Self { schematic_map }
    }
}

impl Schematic {
    pub fn get_part_numbers(&self) -> Vec<u32> {
        let mut result = vec![];
        for entry in self.schematic_map.values() {
            match *entry {
                SchematicEntry::Number { value, first_pos: _, last_pos: _ } => {
                    for cursor in entry.into_iter() {
                        if self.schematic_map.get(&cursor).is_some_and(|x| {
                            match *x {
                                SchematicEntry::Symbol{ pos: _ } => true,
                                SchematicEntry::Gear { pos: _ } => true,
                                _ => false
                            }
                        }) {
                            result.push(value);
                            break;
                        }
                    }
                },
                _ => {}
            }
        }
        result
    }

    pub fn get_gear_ratios(&self) -> Vec<u32> {
        let mut result: Vec<u32> = vec![];
        for entry in self.schematic_map.values().filter(|&e| matches!(e, SchematicEntry::Gear { pos: _ })) {
            let mut current_ratio = 1;
            let mut num_elements_found = 0;
            let mut found_elements = HashSet::new();
            for cursor in entry.into_iter() {
                if let Some(val) = self.schematic_map.get(&cursor) {
                    match *val {
                         SchematicEntry::Number { value, first_pos, last_pos: _ } => {
                            if !found_elements.contains(&first_pos) {
                                found_elements.insert(first_pos);
                                current_ratio *= value;
                                num_elements_found += 1;
                                if num_elements_found > 2 { break; }
                            }
                         },
                         _ => {}
                    }
                }
            }
            if num_elements_found == 2 {
                result.push(current_ratio);
            }
        }
        result
    }

    pub fn _print_numbers(&self) {
        for entry in self.schematic_map.values() {
            match *entry {
                SchematicEntry::Number { value, first_pos, last_pos } => {
                    println!("value: {}, first_pos: {:?}, last_pos: {:?}", value, first_pos, last_pos);
                },
                SchematicEntry::Symbol { pos } => {
                    println!("pos: {:?}", pos);
                },
                _ => {}
            }
        }
    }
}

pub fn run_part_1(input: &mut File) -> u32 {
    let puzzle_input: PuzzleInput<SchematicEntryBuilderLine> = PuzzleInput::from_file(input).unwrap();
    let schematic = Schematic::from(puzzle_input);
    // schematic.print_numbers();
    schematic.get_part_numbers().iter().sum()
}

pub fn run_part_2(input: &mut File) -> u32 {
    let puzzle_input: PuzzleInput<SchematicEntryBuilderLine> = PuzzleInput::from_file(input).unwrap();
    let schematic = Schematic::from(puzzle_input);
    // schematic.print_numbers();
    schematic.get_gear_ratios().iter().sum()
}

#[cfg(test)]
mod test {
    use crate::common::puzzle_input_reader::test::MockFile;

    use super::{ run_part_1, run_part_2 };

    #[test]
    fn test_sample_input_1() {
        let mut input = MockFile::with_contents("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..");
        let output = run_part_1(input.get_file());
        let expected_output = 4361;
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_sample_input_2() {
        let mut input = MockFile::with_contents("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..");
        let output = run_part_2(input.get_file());
        let expected_output = 467835;
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_corners()  {
        let tests = vec![
            (
"....*
.189.
.....",
189
            ),
            (
"*....
.189.
.....",
189
            ),
            (
".....
.189.
*....",
189
            ),
            (
".....
.189.
....*",
189
            ),
            (
"...
.1.
..*",
1
            ),
            (
"...
.1*
...",
1
            ),
            (
"..*
.1.
...",
1
            ),
            (
".*.
.1.
...",
1
            ),
            (
"*..
.1.
...",
1
            ),
            (
"...
*1.
...",
1
            ),
            (
"...
.1.
*..",
1
            ),
            (
"...
.1.
.*.",
1
            ),
            (
"123
4*6
789",
123+4+6+789
            ),
            (
"1.2.3
4*.*6
7.8.9",
1+2+3+4+6+7+8+9
            ),
            (
"......
.-576.
......",
576
            ),
            (
"******&
*.....*
*.576.*
*.....*
*******",
0
            ),
            (
"******&
*.....*
**576.*
*.....*
*******",
576
            ),
            (
"******&
*.....*
*.576.*
**....*
*******",
576
            ),
            (
"******&
*.....*
*.576.*
*....**
*******",
576
            ),
            (
"******&
*.....*
*.576**
*.....*
*******",
576
            ),
            (
"******&
*....**
*.576.*
*.....*
*******",
576
            ),
            (
"******&
**....*
*.576.*
*.....*
*******",
576
            ),
            (
"576",
0
            ),
            (
"**...
..576",
576
            ),
            (
"***...
...576",
576
            ),
            (
"......***
...576...",
576
            ),
            (
"......806.....*....................*...........@................45.....475...724..*......&45.........+202..-576.....*.........*.............
...............383...........................372..................................474...................................432.471......729....",
383+372+474+45+202+576+471
            )
        ];
        for (content, expected_output) in tests {

            let mut input = MockFile::with_contents(content);
            let output = run_part_1(input.get_file());
            assert_eq!(output, expected_output);
        }
    }
}
