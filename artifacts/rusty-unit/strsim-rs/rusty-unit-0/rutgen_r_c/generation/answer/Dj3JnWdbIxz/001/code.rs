// Answer 0

#[test]
fn test_bigrams_empty_string() {
    let result: Vec<(char, char)> = bigrams("").collect();
    assert_eq!(result, vec![]);
}

#[test]
fn test_bigrams_single_character() {
    let result: Vec<(char, char)> = bigrams("a").collect();
    assert_eq!(result, vec![]);
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
    let result: Vec<(char, char)> = bigrams("aaa").collect();
    assert_eq!(result, vec![('a', 'a'), ('a', 'a')]);
}

#[test]
fn test_bigrams_special_characters() {
    let result: Vec<(char, char)> = bigrams("!@#$").collect();
    assert_eq!(result, vec![('!', '@'), ('@', '#'), ('#', '$')]);
}

#[test]
fn test_bigrams_unicode_characters() {
    let result: Vec<(char, char)> = bigrams("汉字").collect();
    assert_eq!(result, vec![('汉', '字')]);
}

