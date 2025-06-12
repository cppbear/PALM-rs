// Answer 0

#[derive(Debug)]
struct LiteralSearcher {
    patterns: Vec<String>,
}

impl LiteralSearcher {
    fn new(patterns: Vec<String>) -> Self {
        Self { patterns }
    }
}

#[derive(Debug)]
struct InputAt {
    position: usize,
}

impl InputAt {
    fn new(position: usize) -> Self {
        Self { position }
    }
}

trait PrefixAt {
    fn prefix_at(&self, prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt>;
}

impl PrefixAt for String {
    fn prefix_at(&self, prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> {
        for pattern in &prefixes.patterns {
            if self.starts_with(pattern) {
                return Some(at);
            }
        }
        None
    }
}

impl std::ops::Deref for String {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        self
    }
}

#[test]
fn test_prefix_at_with_valid_prefix() {
    let input_string = String::from("hello");
    let searcher = LiteralSearcher::new(vec![String::from("he")]);
    let position = InputAt::new(0);

    let result = input_string.prefix_at(&searcher, position);
    assert_eq!(result, Some(InputAt::new(0)));
}

#[test]
fn test_prefix_at_with_no_valid_prefix() {
    let input_string = String::from("hello");
    let searcher = LiteralSearcher::new(vec![String::from("world")]);
    let position = InputAt::new(0);

    let result = input_string.prefix_at(&searcher, position);
    assert_eq!(result, None);
}

#[test]
fn test_prefix_at_with_empty_prefixes() {
    let input_string = String::from("hello");
    let searcher = LiteralSearcher::new(vec![]);
    let position = InputAt::new(0);

    let result = input_string.prefix_at(&searcher, position);
    assert_eq!(result, None);
}

#[test]
fn test_prefix_at_with_multiple_prefixes() {
    let input_string = String::from("hello");
    let searcher = LiteralSearcher::new(vec![String::from("he"), String::from("hello")]);
    let position = InputAt::new(0);

    let result = input_string.prefix_at(&searcher, position);
    assert_eq!(result, Some(InputAt::new(0)));
}

