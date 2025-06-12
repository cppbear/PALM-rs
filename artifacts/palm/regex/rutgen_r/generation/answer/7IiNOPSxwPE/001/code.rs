// Answer 0

#[test]
fn test_expand_str_empty_replacement() {
    use re_unicode::{Captures, Ref};

    struct DummyCaptures {
        data: Vec<Option<&'static str>>,
    }

    impl Captures for DummyCaptures {
        fn get(&self, index: usize) -> Option<&str> {
            self.data.get(index).and_then(|&opt| opt)
        }

        fn name(&self, _name: &str) -> Option<&str> {
            None
        }
    }
    
    let caps = DummyCaptures { data: vec![Some("match1"), Some("match2")] };
    let mut dst = String::new();
    let replacement = "";
    
    expand_str(&caps, replacement, &mut dst);
    
    assert_eq!(dst, "");
}

#[test]
fn test_expand_str_replacement_with_no_captures() {
    use re_unicode::{Captures, Ref};

    struct DummyCaptures {
        data: Vec<Option<&'static str>>,
    }

    impl Captures for DummyCaptures {
        fn get(&self, index: usize) -> Option<&str> {
            self.data.get(index).and_then(|&opt| opt)
        }

        fn name(&self, _name: &str) -> Option<&str> {
            None
        }
    }
    
    let caps = DummyCaptures { data: vec![None, None] };
    let mut dst = String::new();
    let replacement = "Nothing to match $1 here";
    
    expand_str(&caps, replacement, &mut dst);
    
    assert_eq!(dst, "Nothing to match  here");
}

#[test]
fn test_expand_str_no_captures_with_dollar() {
    use re_unicode::{Captures, Ref};

    struct DummyCaptures {
        data: Vec<Option<&'static str>>,
    }

    impl Captures for DummyCaptures {
        fn get(&self, index: usize) -> Option<&str> {
            self.data.get(index).and_then(|&opt| opt)
        }

        fn name(&self, _name: &str) -> Option<&str> {
            None
        }
    }
    
    let caps = DummyCaptures { data: vec![None, None] };
    let mut dst = String::new();
    let replacement = "$$";
    
    expand_str(&caps, replacement, &mut dst);
    
    assert_eq!(dst, "$$");
}

