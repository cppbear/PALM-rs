// Answer 0

#[test]
fn test_bigrams_empty_string() {
    let result: Vec<(char, char)> = bigrams("").collect();
    assert_eq!(result, Vec::<(char, char)>::new());
}

#[test]
fn test_bigrams_single_character() {
    let result: Vec<(char, char)> = bigrams("a").collect();
    assert_eq!(result, Vec::<(char, char)>::new());
}

#[test]
fn test_bigrams_two_characters() {
    let result: Vec<(char, char)> = bigrams("ab").collect();
    assert_eq!(result, vec![('a', 'b')]);
}

#[test]
fn test_bigrams_multiple_characters() {
    let result: Vec<(char, char)> = bigrams("abc").collect();
    assert_eq!(result, vec![('a', 'b'), ('b', 'c')]);
}

#[test]
fn test_bigrams_repeated_characters() {
    let result: Vec<(char, char)> = bigrams("aabb").collect();
    assert_eq!(result, vec![('a', 'a'), ('a', 'b'), ('b', 'b')]);
}

#[test]
fn test_bigrams_whitespace() {
    let result: Vec<(char, char)> = bigrams("  ").collect();
    assert_eq!(result, vec![(' ', ' ')]);
}

