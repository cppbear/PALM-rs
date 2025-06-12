// Answer 0

#[test]
fn test_find_leftmost_first_match() {
    struct TestRegex {
        pattern: &'static str,
    }

    impl TestRegex {
        fn find<'t>(&self, text: &'t str) -> Option<std::ops::Range<usize>> {
            let regex = regex::Regex::new(self.pattern).unwrap();
            regex.find(text).map(|mat| mat.start()..mat.end())
        }
    }

    let regex = TestRegex {
        pattern: r"\b\w{13}\b",
    };
    let text = "I categorically deny having triskaidekaphobia.";
    let result = regex.find(text).unwrap();
    assert_eq!(result.start, 2);
    assert_eq!(result.end, 15);
}

#[test]
fn test_find_no_match() {
    struct TestRegex {
        pattern: &'static str,
    }

    impl TestRegex {
        fn find<'t>(&self, text: &'t str) -> Option<std::ops::Range<usize>> {
            let regex = regex::Regex::new(self.pattern).unwrap();
            regex.find(text).map(|mat| mat.start()..mat.end())
        }
    }

    let regex = TestRegex {
        pattern: r"\b\w{20}\b",
    };
    let text = "This is a short text.";
    let result = regex.find(text);
    assert!(result.is_none());
}

#[test]
fn test_find_boundary_match() {
    struct TestRegex {
        pattern: &'static str,
    }

    impl TestRegex {
        fn find<'t>(&self, text: &'t str) -> Option<std::ops::Range<usize>> {
            let regex = regex::Regex::new(self.pattern).unwrap();
            regex.find(text).map(|mat| mat.start()..mat.end())
        }
    }

    let regex = TestRegex {
        pattern: r"\b\w{1}\b",
    };
    let text = "a b c d e f";
    let result = regex.find(text).unwrap();
    assert_eq!(result.start, 0);
    assert_eq!(result.end, 1);
}

