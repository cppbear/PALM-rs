// Answer 0

#[test]
fn test_replace_append() {
    struct DummyRegex(String);
    
    impl DummyRegex {
        fn new(pattern: &str) -> Self {
            DummyRegex(pattern.to_string())
        }

        fn replace_append(&mut self, _caps: &Captures, dst: &mut String) {
            dst.push_str(&self.0);
        }
    }
    
    struct Captures {
        // Assume we have some fields here relevant for the tests
    }

    let mut regex = DummyRegex::new("pattern");
    let mut destination = String::new();
    let caps = Captures {};

    regex.replace_append(&caps, &mut destination);

    assert_eq!(destination, "pattern");
}

