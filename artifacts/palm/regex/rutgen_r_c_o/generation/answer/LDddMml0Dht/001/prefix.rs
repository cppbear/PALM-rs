// Answer 0

#[test]
fn test_bump_if_valid_prefix_at_start() {
    let parser = ParserI::new(Parser::default(), "hello world");
    parser.offset.set(0);
    let result = parser.bump_if("hello");
}

#[test]
fn test_bump_if_valid_prefix_mid_string() {
    let parser = ParserI::new(Parser::default(), "hello world");
    parser.offset.set(0);
    let result = parser.bump_if("hell");
}

#[test]
fn test_bump_if_valid_prefix_single_character() {
    let parser = ParserI::new(Parser::default(), "a quick brown fox");
    parser.offset.set(0);
    let result = parser.bump_if("a");
}

#[test]
fn test_bump_if_uncommon_valid_prefix() {
    let parser = ParserI::new(Parser::default(), "regex syntax parser");
    parser.offset.set(0);
    let result = parser.bump_if("regex");
}

#[test]
fn test_bump_if_prefix_longer_than_remaining_pattern() {
    let parser = ParserI::new(Parser::default(), "test");
    parser.offset.set(0);
    let result = parser.bump_if("testing");
}

#[test]
fn test_bump_if_empty_prefix() {
    let parser = ParserI::new(Parser::default(), "test input");
    parser.offset.set(0);
    let result = parser.bump_if("");
}

#[test]
fn test_bump_if_non_matching_prefix() {
    let parser = ParserI::new(Parser::default(), "test input");
    parser.offset.set(0);
    let result = parser.bump_if("input");
}

