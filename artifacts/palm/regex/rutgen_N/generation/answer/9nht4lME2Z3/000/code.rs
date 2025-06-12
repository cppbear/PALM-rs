// Answer 0

#[derive(Debug)]
struct Match<'t> {
    text: &'t [u8],
    start: usize,
    end: usize,
}

impl<'t> Match<'t> {
    fn new(text: &'t [u8], start: usize, end: usize) -> Self {
        Match { text, start, end }
    }
}

struct Searcher;

impl Searcher {
    fn find_at(&self, text: &[u8], start: usize) -> Option<(usize, usize)> {
        // Example implementation for testing purposes
        if start < text.len() {
            Some((start, text.len())) // returns a match from 'start' to the end of the text
        } else {
            None
        }
    }
}

struct MyStruct(Searcher);

impl MyStruct {
    pub fn find_at<'t>(&self, text: &'t [u8], start: usize) -> Option<Match<'t>> {
        self.0.find_at(text, start)
            .map(|(s, e)| Match::new(text, s, e))
    }
}

#[test]
fn test_find_at_valid_start() {
    let searcher = MyStruct(Searcher);
    let text = b"hello world";
    let result = searcher.find_at(text, 0);
    assert!(result.is_some());
    if let Some(m) = result {
        assert_eq!(m.start, 0);
        assert_eq!(m.end, text.len());
    }
}

#[test]
fn test_find_at_middle_start() {
    let searcher = MyStruct(Searcher);
    let text = b"hello world";
    let result = searcher.find_at(text, 6);
    assert!(result.is_some());
    if let Some(m) = result {
        assert_eq!(m.start, 6);
        assert_eq!(m.end, text.len());
    }
}

#[test]
fn test_find_at_out_of_bounds_start() {
    let searcher = MyStruct(Searcher);
    let text = b"hello world";
    let result = searcher.find_at(text, 15);
    assert!(result.is_none());
}

