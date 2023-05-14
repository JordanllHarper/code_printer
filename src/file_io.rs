pub mod file_io_mod {
    use std::io::Error;
    use std::{fs, io};
    use std::io::BufRead;

    pub(crate) fn read_lines_in_file(filename: &str) -> Result<String, Error> {
        let file = fs::File::open(filename)?;
        let lines = io::BufReader::new(file).lines();

        let mut complete_line = String::new();

        for line in lines {
            if let Ok(_line) = line {
                complete_line.push_str(&_line);
            }
        }
        Ok(complete_line)
    }
}