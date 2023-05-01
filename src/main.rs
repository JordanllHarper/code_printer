use std::env;

use crate::write_to_word_doc::write_to_word_doc_mod::write_to_word_doc;

mod read_file;
mod write_to_word_doc;

fn build_start_path(start_dir: &str) -> Result<String, std::io::Error> {
    let path = start_dir.to_owned() + "/";
    Ok(path)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = match build_start_path(&args[1]) {
        Ok(c) => c.to_string(),
        Err(e) => {
            println!("Error with building the file path: {}", e.to_string());
            return Err(e);
        }
    };
    println!("Searching {}", path);
    let contents =
        match read_file::read_file_mod::read_files_recursively(&path, "txt", "assign_inc") {
            Ok(contents) => contents,
            Err(_) => "Error".to_string(),
        };

    let result = write_to_word_doc("output.docx", &contents);

    match result {
        Ok(c) => println!("{}", c),
        Err(_) => println!("Something went wrong..."),
    }

    Ok(())
}
