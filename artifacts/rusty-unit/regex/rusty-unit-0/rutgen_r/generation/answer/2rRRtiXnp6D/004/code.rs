// Answer 0

#[test]
fn test_expand_bytes_case_1() {
    use re_bytes::{Captures, Ref};
    use memchr::memchr;

    struct MockCaptures {
        data: Vec<Option<&'static [u8]>>,
    }

    impl Captures for MockCaptures {
        fn get(&self, index: usize) -> Option<&'static str> {
            self.data.get(index).and_then(|opt| opt.map(|&s| unsafe { std::str::from_utf8_unchecked(s) }))
        }
        
        fn name(&self, _: &str) -> Option<&'static str> {
            None
        }
    }

    let mut dst = Vec::new();
    let caps = MockCaptures { data: vec![Some(b"test"), None] };
    let replacement = b"$0 and $1";
    
    expand_bytes(&caps, replacement, &mut dst);
    
    assert_eq!(dst, b"test and ");
}

#[test]
fn test_expand_bytes_case_2() {
    use re_bytes::{Captures, Ref};
    
    struct MockCaptures {
        data: Vec<Option<&'static [u8]>>,
    }

    impl Captures for MockCaptures {
        fn get(&self, index: usize) -> Option<&'static str> {
            self.data.get(index).and_then(|opt| opt.map(|&s| unsafe { std::str::from_utf8_unchecked(s) }))
        }
        
        fn name(&self, _: &str) -> Option<&'static str> {
            None
        }
    }
    
    let mut dst = Vec::new();
    let caps = MockCaptures { data: vec![Some(b"foo"), Some(b"bar")] };
    let replacement = b"$0 and $1";
    
    expand_bytes(&caps, replacement, &mut dst);
    
    assert_eq!(dst, b"foo and bar");
}

#[test]
fn test_expand_bytes_no_placeholder() {
    use re_bytes::{Captures, Ref};
    
    struct MockCaptures {
        data: Vec<Option<&'static [u8]>>,
    }

    impl Captures for MockCaptures {
        fn get(&self, index: usize) -> Option<&'static str> {
            self.data.get(index).and_then(|opt| opt.map(|&s| unsafe { std::str::from_utf8_unchecked(s) }))
        }
        
        fn name(&self, _: &str) -> Option<&'static str> {
            None
        }
    }

    let mut dst = Vec::new();
    let caps = MockCaptures { data: vec![Some(b"hello")] };
    let replacement = b"no placeholders here";
    
    expand_bytes(&caps, replacement, &mut dst);
    
    assert_eq!(dst, b"no placeholders here");
}

#[test]
fn test_expand_bytes_double_dollar() {
    use re_bytes::{Captures, Ref};
    
    struct MockCaptures {
        data: Vec<Option<&'static [u8]>>,
    }

    impl Captures for MockCaptures {
        fn get(&self, index: usize) -> Option<&'static str> {
            self.data.get(index).and_then(|opt| opt.map(|&s| unsafe { std::str::from_utf8_unchecked(s) }))
        }
        
        fn name(&self, _: &str) -> Option<&'static str> {
            None
        }
    }

    let mut dst = Vec::new();
    let caps = MockCaptures { data: vec![None] };
    let replacement = b"$$ and no reference";
    
    expand_bytes(&caps, replacement, &mut dst);
    
    assert_eq!(dst, b"$ and no reference");
}

