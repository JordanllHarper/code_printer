pub mod read_directory_mod {
    use std::fs;
    use crate::node_result::node_result::NodeResult;

    use crate::read_file::read_file_mod::iterate_paths;

    //Entry point + called recursively until all content collected
    pub fn read_from_dir(
        dir_path: &str,
        desired_file_extension: &str,
        file_include_sig: &str,
    ) -> Result<NodeResult, std::io::Error> {
        //create a new node result for collecting the contents of this read_files
        let mut collector_result = NodeResult::new();

        //get all the paths in the file
        let paths = match fs::read_dir(dir_path) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };


        let content_in_paths = iterate_paths(paths, desired_file_extension, file_include_sig);


        //add the contents to this collector result
        if content_in_paths.include_content {
            collector_result.include_content = true;
            collector_result.contents += &content_in_paths.contents;
            return Ok(collector_result);
        }

        return Ok(NodeResult::new());
    }
}