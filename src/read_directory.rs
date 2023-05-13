pub mod read_directory_mod{

    use std::fs;
    use crate::node_result::node_result::NodeResult;
    use crate::read_file::read_file_mod;
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


        let found_content = match iterate_paths(paths, desired_file_extension, file_include_sig) {
            Ok(r) => {
                collector_result.include_content = r.include_content;
                r
            }
            Err(_) => NodeResult::new(),
        };

        //add the contents to this collector result
        collector_result.contents += &found_content.contents;
        //if we want to include the content in this result, propagate up true
        if collector_result.include_content {
            return Ok(collector_result);
        }

        return Ok(NodeResult::new());
    }
}