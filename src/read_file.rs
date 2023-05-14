pub mod read_file_mod {
    use crate::file_io::file_io_mod::read_lines_in_file;


    //handles the case where a file isn't a directory
    pub fn handle_non_dir(path: &str, delimit: Vec<&str>, desired_file_extension: &str) -> Option<String> {
        //isn't a directory so treat as a file

        //guard clauses
        //make sure that the file has a . extension
        //TODO: Support non . files in the future
        if delimit.len() != 2 {
            return None;
        };
        //check if the file extension is what we care about
        if delimit[1] != desired_file_extension {
            return None;
        }
        //we passed all the guard clauses and we care about this file
        match read_lines_in_file(path) {
            Ok(c) => Some(c),
            Err(_) => None,
        }
    }
}