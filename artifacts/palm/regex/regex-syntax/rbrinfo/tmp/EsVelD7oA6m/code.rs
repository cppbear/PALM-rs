pub fn is_word_character(c: char) -> bool {
    use std::cmp::Ordering;
    use unicode_tables::perl_word::PERL_WORD;

    if c <= 0x7F as char && is_word_byte(c as u8) {
        return true;
    }
    PERL_WORD
        .binary_search_by(|&(start, end)| {
            if start <= c && c <= end {
                Ordering::Equal
            } else if start > c {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }).is_ok()
}