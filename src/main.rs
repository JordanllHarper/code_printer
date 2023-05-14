use std::env;

use crate::{
    write_to_word_doc::write_to_word_doc_mod::write_to_word_doc,
};
use crate::read_directory::read_directory_mod::read_from_dir;

mod node_result;
mod read_file;
mod write_to_word_doc;
mod read_directory;

fn build_start_path(start_dir: &str) -> Result<String, std::io::Error> {
    let path = start_dir.to_owned() + "/";
    Ok(path)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("CODE PRINTER: => Invalid number of arguments\nFormat is: '[os_dependent_exe_file_run]\n [path/to/start_dir]\n [file_type_to_write e.g. txt or rs]\n [opt: dir/to/save]' ");
        return Ok(());
    }
    let path = match build_start_path(&args[1]) {
        Ok(c) => c.to_string(),
        Err(e) => {
            println!("Error with building the file path: {}", e.to_string());
            return Err(e);
        }
    };

    let file_type_to_search = &args[2];

    let mut path_to_save = ".";
    if args.len() == 4 {
        path_to_save = &args[3];
    }

    println!("Searching {}", path);
    let result = match read_from_dir(&path, file_type_to_search, "assign_inc") {
        Ok(contents) => contents,
        Err(e) => return Err(e),
    };

    if !result.include_content{
        println!("Found no assign_inc files so nothing is written .");
        return Ok(());
    }

    let contents = result.contents;


    if contents.trim().is_empty() {
        println!("No contents to print so not creating the file.");
        return Ok(());
    }
    write_to_word_doc(&(path_to_save.to_string() + "/output.docx"), &contents);


    Ok(())
}
