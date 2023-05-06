pub mod read_file_system_mod {
    use std::{
        fs,
        io::{Error, Result},
    };

    use crate::ReadFileResult::ReadFileResultMod::ReadFileResult;
    /// Reads an individ directory
    /// If finds another directory, recursively recalls
    pub fn read_dir(
        dir_path: &str,
        desired_file_extension: &str,
        file_include_sig: &str,
    ) -> Result<ReadFileResult, Error> {
        let result = ReadFileResult {
            include_content: false,
            contents: String::new(),
        };
        let paths = match fs::read_dir(dir_path) {
            Ok(p) => p,
            Err(_) => {
                return Ok(ReadFileResult {
                    include_content: false,
                    contents: String::new(),
                })
            }
        };
        for path in paths {
            let file_type = match path.unwrap().file_type() {
                Ok(t) => t,
                Err(e) => return Err(e),
            };
            if file_type.is_dir() {
                //TODO: Do something with this
                let future_dir_result = read_dir(
                    &path.unwrap().path().to_str(),
                    &desired_file_extension,
                    &file_include_sig,
                );
            }
        }

        Ok(result)
    }

    fn read_file() -> String {}
}
