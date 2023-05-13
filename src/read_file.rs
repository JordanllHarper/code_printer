pub mod read_file_mod {
    use std::{
        fs::{self, ReadDir},
        io::{self, BufRead, Error},
    };
    use std::fs::DirEntry;


    use crate::node_result::node_result::NodeResult;
    use crate::read_directory::read_directory_mod::read_from_dir;

    fn read_lines_in_file(filename: &str) -> Result<String, Error> {
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


    //handles the case where a file isn't a directory
    fn handle_non_dir(path: &str, delimit: Vec<&str>, desired_file_extension: &str) -> Option<String> {
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


    fn handle_each_path(dir_entry: Result<DirEntry, Error>, desired_file_extension: &str, file_include_sig: &str) -> Option<NodeResult> {
        let mut each_path_node_collector = NodeResult::new();
        let path = match dir_entry {
            Ok(p) => p,
            Err(_) => return None,
        }
            .path();

        let delimit: Vec<&str> = path
            .to_str()
            .unwrap_or_default()
            .split(".")
            .collect();

        if path.is_dir() {
            let path_str = match path
                .to_str() {
                Some(c) => c,
                None => return None,
            };

            //add all the contents of the child to this node result
            each_path_node_collector.contents +=
                //the recursive call back to read_from_dir
                &match read_from_dir(path_str, desired_file_extension, file_include_sig) {
                    Ok(c) => c.contents,
                    Err(_) => "".to_string(),
                };

        } else {
            //check if the file is the marking file for including content in the directory
            if delimit[0] == file_include_sig {
                each_path_node_collector.include_content = true
            }
            let path = match path.to_str() {
                Some(p) => p,
                None => return None,
            };

            let file_content = match handle_non_dir(path, delimit, desired_file_extension) {
                Some(c) => c,
                None => "".to_string()
            };


            each_path_node_collector.contents += &file_content;
        }
        Some(each_path_node_collector)
    }

    pub fn iterate_paths(
        paths: ReadDir,
        desired_file_extension: &str,
        file_include_sig: &str,
    ) -> NodeResult {
        //Define an initial collector result for this parse
        let mut collector_result = NodeResult::new();

        for path in paths {
            let node_result = match handle_each_path(
                path,
                desired_file_extension,
                file_include_sig) {

                Some(c) => c,
                None => continue,

            };

            collector_result.include_content = node_result.include_content;
            if node_result.include_content {
                collector_result.contents += &node_result.contents
            }
        }
        return collector_result;
    }
}