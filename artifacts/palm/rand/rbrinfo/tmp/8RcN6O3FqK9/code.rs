fn char_to_comp_u32(c: char) -> u32 {
    match c as u32 {
        c if c >= CHAR_SURROGATE_START => c - CHAR_SURROGATE_LEN,
        c => c,
    }
}