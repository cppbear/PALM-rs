fn is_capture_char(c: char, first: bool) -> bool {
    c == '_' || (!first && c >= '0' && c <= '9')
    || (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}