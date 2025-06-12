pub fn escape_into(text: &str, buf: &mut String) {
    for c in text.chars() {
        if is_meta_character(c) {
            buf.push('\\');
        }
        buf.push(c);
    }
}