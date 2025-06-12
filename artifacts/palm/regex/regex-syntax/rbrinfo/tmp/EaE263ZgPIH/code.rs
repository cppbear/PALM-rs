pub fn escape(text: &str) -> String {
    let mut quoted = String::with_capacity(text.len());
    escape_into(text, &mut quoted);
    quoted
}