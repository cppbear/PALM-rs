fn char_len_lossy(bytes: &[u8]) -> usize {
    String::from_utf8_lossy(bytes).chars().count()
}