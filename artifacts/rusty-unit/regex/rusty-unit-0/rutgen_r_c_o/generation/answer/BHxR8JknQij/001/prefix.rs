// Answer 0

#[test]
fn test_splitn_empty_string_limit_zero() {
    let re = Regex::new(r"\W+").unwrap();
    let _result = re.splitn("", 0);
}

#[test]
fn test_splitn_empty_string_limit_five() {
    let re = Regex::new(r"\W+").unwrap();
    let _result = re.splitn("", 5);
}

#[test]
fn test_splitn_single_word_limit_one() {
    let re = Regex::new(r"\W+").unwrap();
    let _result = re.splitn("abc", 1);
}

#[test]
fn test_splitn_multiple_words_limit_two() {
    let re = Regex::new(r"\W+").unwrap();
    let _result = re.splitn("abc def", 2);
}

#[test]
fn test_splitn_special_characters_limit_three() {
    let re = Regex::new(r"\W+").unwrap();
    let _result = re.splitn("123!@#", 3);
}

#[test]
fn test_splitn_few_words_limit_three() {
    let re = Regex::new(r"\W+").unwrap();
    let _result = re.splitn("one two three", 3);
}

#[test]
fn test_splitn_multiple_characters_limit_five() {
    let re = Regex::new(r"\W+").unwrap();
    let _result = re.splitn("a b c d e f", 5);
}

#[test]
fn test_splitn_limit_exceeding_word_count() {
    let re = Regex::new(r"\W+").unwrap();
    let _result = re.splitn("hay there everyone", 10);
} 

#[test]
fn test_splitn_limit_zero_non_empty() {
    let re = Regex::new(r"\W+").unwrap();
    let _result = re.splitn("Hello World", 0);
}

