// Answer 0

#[derive(Debug)]
struct Span {
    length: usize,
}

struct Joins {
    items: Vec<String>,
}

impl Joins {
    fn join(&self) -> Span {
        Span {
            length: self.items.iter().map(|s| s.len()).sum(),
        }
    }

    fn new(items: Vec<String>) -> Self {
        Joins { items }
    }
}

#[test]
fn test_empty_join() {
    let joins = Joins::new(vec![]);
    let span = joins.join();
    assert_eq!(span.length, 0);
}

#[test]
fn test_single_item_join() {
    let joins = Joins::new(vec!["test".to_string()]);
    let span = joins.join();
    assert_eq!(span.length, 4);
}

#[test]
fn test_multiple_item_join() {
    let joins = Joins::new(vec!["Hello".to_string(), " ".to_string(), "World".to_string()]);
    let span = joins.join();
    assert_eq!(span.length, 11);
}

#[test]
fn test_special_characters_join() {
    let joins = Joins::new(vec!["!@#".to_string(), "$%^".to_string(), "&*()".to_string()]);
    let span = joins.join();
    assert_eq!(span.length, 12);
}

#[test]
#[should_panic]
fn test_panics_on_large_items() {
    let long_string = "a".repeat(usize::MAX);
    let joins = Joins::new(vec![long_string]);
    let _span = joins.join(); // This may panic based on system configuration.
}

