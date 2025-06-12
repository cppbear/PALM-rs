fn eq_ignore_ascii_case(lower: &[u8], s: &[u8]) -> bool {
    if lower.len() != s.len() {
        return false;
    }

    lower
        .iter()
        .zip(s)
        .all(|(a, b)| *a == HEADER_CHARS[*b as usize])
}