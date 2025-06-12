fn repeat_char(c: char, count: usize) -> String {
    ::std::iter::repeat(c).take(count).collect()
}