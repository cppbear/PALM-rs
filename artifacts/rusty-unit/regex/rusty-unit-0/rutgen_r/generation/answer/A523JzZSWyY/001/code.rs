// Answer 0

#[test]
fn test_find_at_start_zero() {
    struct Searcher(&'static str);

    impl Searcher {
        fn searcher_str(&self) -> &str {
            self.0
        }
    }

    let searcher = Searcher("test");
    let result = searcher.find_at("test string", 0);
    assert!(result.is_some());
    assert_eq!(result.unwrap().start, 0);
    assert_eq!(result.unwrap().end, 4);
}

#[test]
fn test_find_at_start_non_zero() {
    struct Searcher(&'static str);

    impl Searcher {
        fn searcher_str(&self) -> &str {
            self.0
        }
    }

    let searcher = Searcher("string");
    let result = searcher.find_at("test string", 5);
    assert!(result.is_some());
    assert_eq!(result.unwrap().start, 5);
    assert_eq!(result.unwrap().end, 11);
}

#[test]
fn test_find_at_start_greater_than_length() {
    struct Searcher(&'static str);

    impl Searcher {
        fn searcher_str(&self) -> &str {
            self.0
        }
    }

    let searcher = Searcher("string");
    let result = searcher.find_at("test string", 20);
    assert!(result.is_none());
}

#[test]
fn test_find_at_start_negative() {
    struct Searcher(&'static str);

    impl Searcher {
        fn searcher_str(&self) -> &str {
            self.0
        }
    }

    let searcher = Searcher("string");
    let result = searcher.find_at("test string", usize::MAX);
    assert!(result.is_none());
}

#[test]
#[should_panic]
fn test_find_at_start_panic_condition() {
    struct Searcher(&'static str);

    impl Searcher {
        fn searcher_str(&self) -> &str {
            self.0
        }
    }

    let searcher = Searcher("string");
    let _ = searcher.find_at("test string", usize::MAX + 1);
}

