fn bigrams(s: &str) -> impl Iterator<Item = (char, char)> + '_ {
    s.chars().zip(s.chars().skip(1))
}