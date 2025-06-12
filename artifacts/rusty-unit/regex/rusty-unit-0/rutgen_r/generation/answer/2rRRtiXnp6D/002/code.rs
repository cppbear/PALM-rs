// Answer 0

#[test]
fn test_expand_bytes_simple_case() {
    use re_bytes::{Captures, Ref};

    struct TestCaptures {
        data: Vec<u8>,
    }

    impl TestCaptures {
        fn new(captures: Vec<&str>) -> Captures {
            let caps = captures.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
            Captures::from_iter(caps)
        }

        fn get(&self, index: usize) -> Option<&[u8]> {
            self.data.get(index).map(|&s| s)
        }
        
        fn name(&self, _name: &str) -> Option<&[u8]> {
            None
        }
    }

    let caps = TestCaptures::new(vec!["hello", "world"]);
    
    let mut replacement: &[u8] = b"this is $0 and not $1";
    let mut dst: Vec<u8> = Vec::new();

    expand_bytes(&caps, replacement, &mut dst);
    
    assert_eq!(dst, b"this is hello and not world");
}

#[test]
fn test_expand_bytes_no_captures() {
    use re_bytes::{Captures, Ref};

    struct TestCaptures {
        data: Vec<u8>,
    }

    impl TestCaptures {
        fn new(captures: Vec<&str>) -> Captures {
            let caps = captures.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
            Captures::from_iter(caps)
        }

        fn get(&self, index: usize) -> Option<&[u8]> {
            self.data.get(index).map(|&s| s)
        }

        fn name(&self, _name: &str) -> Option<&[u8]> {
            None
        }
    }

    let caps = TestCaptures::new(vec![]);

    let mut replacement: &[u8] = b"Check $0 and $1.";
    let mut dst: Vec<u8> = Vec::new();

    expand_bytes(&caps, replacement, &mut dst);
    
    assert_eq!(dst, b"Check $ and $.");
}

#[test]
#[should_panic]
fn test_expand_bytes_invalid_access() {
    use re_bytes::{Captures, Ref};

    struct TestCaptures {
        data: Vec<u8>,
    }

    impl TestCaptures {
        fn new(captures: Vec<&str>) -> Captures {
            let caps = captures.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
            Captures::from_iter(caps)
        }

        fn get(&self, index: usize) -> Option<&[u8]> {
            self.data.get(index).map(|&s| s)
        }

        fn name(&self, _name: &str) -> Option<&[u8]> {
            None
        }
    }

    let caps = TestCaptures::new(vec!["abc"]);

    let mut replacement: &[u8] = b"Replace $2";
    let mut dst: Vec<u8> = Vec::new();

    expand_bytes(&caps, replacement, &mut dst);
}

#[test]
fn test_expand_bytes_named_capture() {
    use re_bytes::{Captures, Ref};

    struct TestCaptures {
        data: Vec<u8>,
    }

    impl TestCaptures {
        fn new(captures: Vec<&str>) -> Captures {
            let caps = captures.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
            Captures::from_iter(caps)
        }

        fn get(&self, index: usize) -> Option<&[u8]> {
            self.data.get(index).map(|&s| s)
        }

        fn name(&self, name: &str) -> Option<&[u8]> {
            if name == "greeting" {
                return Some(b"hello");
            }
            None
        }
    }

    let caps = TestCaptures::new(vec![]);

    let mut replacement: &[u8] = b"Say $greeting.";
    let mut dst: Vec<u8> = Vec::new();

    expand_bytes(&caps, replacement, &mut dst);
    
    assert_eq!(dst, b"Say hello.");
}

