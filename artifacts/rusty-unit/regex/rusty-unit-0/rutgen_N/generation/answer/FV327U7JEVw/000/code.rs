// Answer 0

#[derive(Debug)]
struct LiteralSearcher {
    // Assuming we have some fields representing the prefixes
    prefixes: Vec<String>,
}

impl LiteralSearcher {
    fn find(&self, input: &str) -> Option<(String, usize)> {
        for prefix in &self.prefixes {
            if input.starts_with(prefix) {
                return Some((prefix.clone(), prefix.len()));
            }
        }
        None
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

struct Input {
    data: String,
}

impl Input {
    fn at(&self, pos: usize) -> InputAt {
        InputAt { position: pos }
    }
    
    fn prefix_at(&self, prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> {
        prefixes.find(&self.data[at.pos()..]).map(|(s, _)| self.at(at.pos() + s.len()))
    }
}

#[test]
fn test_prefix_at_found() {
    let input = Input { data: String::from("hello world") };
    let searcher = LiteralSearcher { prefixes: vec![String::from("hello")] };
    let at = InputAt { position: 0 };
    
    let result = input.prefix_at(&searcher, at);
    assert!(result.is_some());
    assert_eq!(result.unwrap().pos(), 5); // "hello" is 5 characters long
}

#[test]
fn test_prefix_at_not_found() {
    let input = Input { data: String::from("hello world") };
    let searcher = LiteralSearcher { prefixes: vec![String::from("bye")] };
    let at = InputAt { position: 0 };
    
    let result = input.prefix_at(&searcher, at);
    assert!(result.is_none());
}

#[test]
fn test_prefix_at_boundary() {
    let input = Input { data: String::from("goodbye") };
    let searcher = LiteralSearcher { prefixes: vec![String::from("good"), String::from("bye")] };
    let at = InputAt { position: 0 };
    
    let result = input.prefix_at(&searcher, at);
    assert!(result.is_some());
    assert_eq!(result.unwrap().pos(), 4); // "good" is 4 characters long
}

