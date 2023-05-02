pub mod read_file_mod {

    use std::{
        fs,
        io::{self, BufRead, Error},
    };

    fn read_lines_in_file(filename: &str) -> Result<String, std::io::Error> {
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

    fn get_paths(dir_path: &str) -> Result<fs::ReadDir, Error> {
        fs::read_dir(dir_path)
    }

    fn file_is_dir(dir_path: &str, desired_file_extension: &str, file_include_sig: &str) -> String {
        match read_files_recursively(dir_path, desired_file_extension, file_include_sig) {
            Ok(c) => c,
            Err(_) => return "".to_string(),
        }
    }

    pub fn read_files_recursively(
        dir_path: &str,
        desired_file_extension: &str,
        file_include_sig: &str,
    ) -> Result<String, std::io::Error> {
        let mut content = String::new();

        let paths = match get_paths(dir_path) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        let mut include_content = false;

        for path in paths {
            let path = path.unwrap();
            println!("Searching {}", path.path().display());
            let file_type = &path.file_type().unwrap();

            if file_type.is_dir() {
                content += &file_is_dir(
                    &path.path().to_str().unwrap(),
                    desired_file_extension,
                    file_include_sig,
                );
            } else {
                let file_name = &path.file_name();
                let file_split: Vec<&str> = file_name.to_str().unwrap().split(".").collect();

                if file_split[0] == file_include_sig {
                    print!("Including content in directory!\n");
                    include_content = true;
                }

                if file_split[1] == desired_file_extension {
                    println!("Found desired file extension: {}", file_split[1]);
                    content += &read_lines_in_file(&path.path().to_str().unwrap()).unwrap();
                    content += "\n";
                }
            }
        }

        match include_content {
            true => Ok(content),
            false => {
                println!("Not including content...");

                Ok("".to_string())
            }
        }
    }
}
