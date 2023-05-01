pub mod write_to_word_doc_mod {

    use docx_rs::*;

    pub fn write_to_word_doc(path: &str, contents: &str) -> String {
        let _path = std::path::Path::new(path);
        let file = std::fs::File::create(_path).unwrap();
        let para = Paragraph::new().add_run(
            Run::new()
                .add_text(contents)
                .fonts(RunFonts::new().ascii("Courier New")),
        );
        let result = Docx::new().add_paragraph(para).build().pack(file);

        return match result {
            Ok(_) => "Successful write".to_string(),
            Err(_) => "Not successful write".to_string(),
        };
    }
}
