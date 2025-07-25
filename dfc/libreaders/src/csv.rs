use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

/// An iterator over the lines of a CSV file.
pub struct CsvIterator {
    // This wraps the file reader and provides the line-by-line iteration.
    lines: Lines<BufReader<File>>,
}

impl CsvIterator {
    /// Creates a new CSV iterator for the given file path.
    ///
    /// It returns an I/O error if the file cannot be opened.
    pub fn new(buf_reader: BufReader<File>) -> io::Result<Self> {
        Ok(Self {
            lines: buf_reader.lines(),
        })
    }
}

/// Implements the standard Iterator trait.
impl Iterator for CsvIterator {
    // Our iterator will yield Results containing Strings.
    type Item = io::Result<String>;

    /// This method is called for each iteration.
    ///
    /// It lazily reads the next line from the file.
    /// Returns `Some(Ok(line))` if a line is read successfully,
    /// `Some(Err(e))` if an I/O error occurs,
    /// and `None` when the end of the file is reached.
    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next()
    }
}

fn main() {

    match CsvIterator::new(file_path) {
        Ok(csv_iterator) => {
            for line_result in csv_iterator {
                match line_result {
                    Ok(line) => println!("  {}", line),
                    Err(e) => eprintln!("Error reading a line: {}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open file '{}': {}", file_path, e);
        }
    }
}
