// Answer 0

#[test]
fn test_expand_str_with_valid_captures() {
    struct MockCaptures {
        values: Vec<Option<String>>,
    }

    impl re_unicode::Captures for MockCaptures {
        fn get(&self, idx: usize) -> Option<re_unicode::Match<'_>> {
            self.values.get(idx).and_then(|val| val.as_ref()).map(|s| re_unicode::Match::new(s))
        }
        
        fn name(&self, _name: &str) -> Option<re_unicode::Match<'_>> {
            None // For simplicity, not implementing named captures in this test
        }
    }

    let caps = MockCaptures {
        values: vec![Some("first".to_string()), Some("second".to_string())],
    };

    let mut output = String::new();
    let replacement = "$0 and $1";
    
    expand_str(&caps, replacement, &mut output);
    
    assert_eq!(output, "first and second");
}

#[test]
fn test_expand_str_with_double_dollar() {
    struct MockCaptures {
        values: Vec<Option<String>>,
    }

    impl re_unicode::Captures for MockCaptures {
        fn get(&self, idx: usize) -> Option<re_unicode::Match<'_>> {
            self.values.get(idx).and_then(|val| val.as_ref()).map(|s| re_unicode::Match::new(s))
        }
        
        fn name(&self, _name: &str) -> Option<re_unicode::Match<'_>> {
            None // For simplicity, not implementing named captures in this test
        }
    }

    let caps = MockCaptures {
        values: vec![Some("value".to_string())],
    };

    let mut output = String::new();
    let replacement = "$$0 is the same as $$1";
    
    expand_str(&caps, replacement, &mut output);
    
    assert_eq!(output, "$value is the same as $");
}

#[test]
fn test_expand_str_with_non_existent_capture() {
    struct MockCaptures {
        values: Vec<Option<String>>,
    }

    impl re_unicode::Captures for MockCaptures {
        fn get(&self, idx: usize) -> Option<re_unicode::Match<'_>> {
            self.values.get(idx).and_then(|val| val.as_ref()).map(|s| re_unicode::Match::new(s))
        }
        
        fn name(&self, _name: &str) -> Option<re_unicode::Match<'_>> {
            None // For simplicity, not implementing named captures in this test
        }
    }

    let caps = MockCaptures {
        values: vec![None, Some("second".to_string())],
    };

    let mut output = String::new();
    let replacement = "$0 and $1 and $2"; // $2 does not exist
    
    expand_str(&caps, replacement, &mut output);
    
    assert_eq!(output, " and second");
}

#[test]
fn test_expand_str_empty_replacement() {
    struct MockCaptures {
        values: Vec<Option<String>>,
    }

    impl re_unicode::Captures for MockCaptures {
        fn get(&self, idx: usize) -> Option<re_unicode::Match<'_>> {
            self.values.get(idx).and_then(|val| val.as_ref()).map(|s| re_unicode::Match::new(s))
        }
        
        fn name(&self, _name: &str) -> Option<re_unicode::Match<'_>> {
            None // For simplicity, not implementing named captures in this test
        }
    }

    let caps = MockCaptures {
        values: vec![Some("only".to_string())],
    };

    let mut output = String::new();
    let replacement = ""; // Testing with empty replacement
    
    expand_str(&caps, replacement, &mut output);
    
    assert_eq!(output, "");
}

