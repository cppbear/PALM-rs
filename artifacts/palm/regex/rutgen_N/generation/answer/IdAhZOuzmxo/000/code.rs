// Answer 0

#[derive(Debug)]
struct LiteralSearcher {
    // Assuming LiteralSearcher has some kind of pattern we can use
    pattern: String,
}

impl LiteralSearcher {
    fn find(&self, input: &str) -> Option<(String, usize)> {
        if input.contains(&self.pattern) {
            Some((self.pattern.clone(), input.find(&self.pattern).unwrap()))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct InputAt {
    position: usize,
}

impl InputAt {
    fn pos(&self) -> usize {
        self.position
    }
}

trait Input {
    fn at(&self, pos: usize) -> InputAt;
}

#[derive(Debug)]
struct MyInput {
    data: String,
}

impl Input for MyInput {
    fn at(&self, pos: usize) -> InputAt {
        InputAt { position: pos }
    }
}

fn prefix_at(
    input: &MyInput,
    prefixes: &LiteralSearcher,
    at: InputAt,
) -> Option<InputAt> {
    prefixes.find(&input.data[at.pos()..]).map(|(s, _)| input.at(at.pos() + s.len()))
}

#[test]
fn test_prefix_at_found() {
    let input = MyInput { data: String::from("hello world") };
    let searcher = LiteralSearcher { pattern: String::from("hello") };
    let at = InputAt { position: 0 };

    let result = prefix_at(&input, &searcher, at);
    assert!(result.is_some());
    assert_eq!(result.unwrap().pos(), 5); // "hello" length is 5
}

#[test]
fn test_prefix_at_not_found() {
    let input = MyInput { data: String::from("hello world") };
    let searcher = LiteralSearcher { pattern: String::from("goodbye") };
    let at = InputAt { position: 0 };

    let result = prefix_at(&input, &searcher, at);
    assert!(result.is_none());
}

#[test]
fn test_prefix_at_empty_input() {
    let input = MyInput { data: String::from("") };
    let searcher = LiteralSearcher { pattern: String::from("hello") };
    let at = InputAt { position: 0 };

    let result = prefix_at(&input, &searcher, at);
    assert!(result.is_none());
}

#[test]
fn test_prefix_at_boundary_condition() {
    let input = MyInput { data: String::from("abc") };
    let searcher = LiteralSearcher { pattern: String::from("abc") };
    let at = InputAt { position: 0 };

    let result = prefix_at(&input, &searcher, at);
    assert!(result.is_some());
    assert_eq!(result.unwrap().pos(), 3); // "abc" length is 3
}

