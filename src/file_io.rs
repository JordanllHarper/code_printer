pub mod file_io_mod {
    use std::fs;

    pub(crate) fn read_lines_in_file(filename: &str) -> Result<String, std::io::Error> {
        let contents = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };

        Ok(contents)
    }
}

