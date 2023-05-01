pub mod write_to_word_doc_mod {

    use docx::document::Paragraph;
    use docx::Docx;
    use docx::DocxError;
    pub fn write_to_word_doc(filename: &str, contents: &str) -> Result<String, DocxError> {
        let mut docx = Docx::default();

        let paragraph = Paragraph::default().push_text(contents);
        docx.document.push(paragraph);
        let result = docx.write_file(filename);
        match result {
            Ok(_) => return Ok("Written code to ".to_owned() + filename),
            Err(e) => return Err(e),
        }
    }
}
