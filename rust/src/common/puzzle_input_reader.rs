use std::fs::File;
use std::error::Error;
use std::io::Read;

pub struct PuzzleInput<T>
where T: From<String> {
    lines: Vec<T>,
}

pub fn read_file(file_name: &str) -> File {
    File::open(file_name).unwrap()
}

impl<T> PuzzleInput<T>
where T: From<String> {

    pub fn from_file(file: &mut File) -> Result<Self, Box<dyn Error>> {
        // let contents = fs::read_to_string(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let lines = contents.split("\n").filter(|s| s.len() > 0).map(|s| s.to_string().into()).collect::<Vec<T>>();

        Ok(Self {
            lines,
        })
    }
}

impl<'a, T> IntoIterator for &'a PuzzleInput<T>
where T: From<String> {
    type Item = &'a T;
    type IntoIter = PuzzleInputIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        PuzzleInputIterator {
            current_line: 0,
            puzzle_input: self
        }
    }
}

pub struct PuzzleInputIterator<'a, T>
where T: From<String> {
    current_line: usize,
    puzzle_input: &'a PuzzleInput<T>
}

impl<'a, T> Iterator for PuzzleInputIterator<'a, T>
where T: From<String> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current_line;

        self.current_line += 1;
        if self.current_line > self.puzzle_input.lines.len() {
            None
        } else {
            self.puzzle_input.lines.get(current)
        }

        // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
        // will never return `None`, and `Some` is always returned.
    }
}

#[cfg(test)]
pub mod test {
    use std::{fs::File, io::{ SeekFrom, Write, Seek }};
    use tempfile;

    use super::PuzzleInput;
    pub struct MockFile {
        inner: File,
    }

    impl MockFile {
        pub fn with_contents(s: &str) -> Self {
            let mut tmp_file = tempfile::tempfile().unwrap();
            write!(tmp_file, "{s}").unwrap();
            tmp_file.seek(SeekFrom::Start(0)).unwrap();
            MockFile { inner: tmp_file }
        }

        pub fn get_file(&mut self) -> &mut File {
            &mut self.inner
        }
    }

    #[test]
    fn test_simple_read() {
        let mut file = MockFile::with_contents("line 1\nline 2");
        let input: PuzzleInput<String> = PuzzleInput::from_file(&mut file.inner).unwrap();

        let expected_output = vec![String::from("line 1"), String::from("line 2")];
        for test_case in input.into_iter().zip(expected_output) {
            assert_eq!(*test_case.0, test_case.1);
        }
    }
}
