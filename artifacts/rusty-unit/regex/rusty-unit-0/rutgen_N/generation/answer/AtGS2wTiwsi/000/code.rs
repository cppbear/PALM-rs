// Answer 0

#[test]
fn test_find_start_exact_match() {
    struct Literal<'a> {
        pattern: &'a [u8],
    }
    
    impl<'a> Literal<'a> {
        fn iter(&self) -> std::slice::Iter<'a, u8> {
            self.pattern.iter()
        }
        
        fn len(&self) -> usize {
            self.pattern.len()
        }
    }

    let literal = Literal { pattern: b"hello" };
    let haystack = b"hello world";
    assert_eq!(literal.find_start(haystack), Some((0, 5)));
}

#[test]
fn test_find_start_no_match() {
    struct Literal<'a> {
        pattern: &'a [u8],
    }
    
    impl<'a> Literal<'a> {
        fn iter(&self) -> std::slice::Iter<'a, u8> {
            self.pattern.iter()
        }
        
        fn len(&self) -> usize {
            self.pattern.len()
        }
    }

    let literal = Literal { pattern: b"goodbye" };
    let haystack = b"hello world";
    assert_eq!(literal.find_start(haystack), None);
}

#[test]
fn test_find_start_empty_haystack() {
    struct Literal<'a> {
        pattern: &'a [u8],
    }
    
    impl<'a> Literal<'a> {
        fn iter(&self) -> std::slice::Iter<'a, u8> {
            self.pattern.iter()
        }
        
        fn len(&self) -> usize {
            self.pattern.len()
        }
    }

    let literal = Literal { pattern: b"test" };
    let haystack: &[u8] = b"";
    assert_eq!(literal.find_start(haystack), None);
}

#[test]
fn test_find_start_pattern_longer_than_haystack() {
    struct Literal<'a> {
        pattern: &'a [u8],
    }
    
    impl<'a> Literal<'a> {
        fn iter(&self) -> std::slice::Iter<'a, u8> {
            self.pattern.iter()
        }
        
        fn len(&self) -> usize {
            self.pattern.len()
        }
    }

    let literal = Literal { pattern: b"longpattern" };
    let haystack = b"short";
    assert_eq!(literal.find_start(haystack), None);
}

#[test]
fn test_find_start_partial_match() {
    struct Literal<'a> {
        pattern: &'a [u8],
    }
    
    impl<'a> Literal<'a> {
        fn iter(&self) -> std::slice::Iter<'a, u8> {
            self.pattern.iter()
        }
        
        fn len(&self) -> usize {
            self.pattern.len()
        }
    }

    let literal = Literal { pattern: b"hello" };
    let haystack = b"hellohello";
    assert_eq!(literal.find_start(haystack), Some((0, 5)));
}

