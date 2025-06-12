pub fn is_meta_character(c: char) -> bool {
    match c {
        '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' |
        '[' | ']' | '{' | '}' | '^' | '$' | '#' | '&' | '-' | '~' => true,
        _ => false,
    }
}