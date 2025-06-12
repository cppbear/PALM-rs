// Answer 0

#[test]
fn test_at_zero() {
    struct TestInput;
    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: Char, byte: None, len: 1 }
        }
        fn len(&self) -> usize { 1 }
        fn as_bytes(&self) -> &[u8] { b"a" }
    }
    
    let input = TestInput;
    let result = input.at(0);
}

#[test]
fn test_at_mid_range() {
    struct TestInput;
    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: Char, byte: None, len: 1 }
        }
        fn len(&self) -> usize { 10 }
        fn as_bytes(&self) -> &[u8] { b"abcdefghij" }
    }
    
    let input = TestInput;
    let result = input.at(5);
}

#[test]
fn test_at_boundary_at_len() {
    struct TestInput;
    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: Char, byte: None, len: 1 }
        }
        fn len(&self) -> usize { 10 }
        fn as_bytes(&self) -> &[u8] { b"abcdefghij" }
    }
    
    let input = TestInput;
    let result = input.at(10);
}

#[test]
fn test_at_out_of_bounds() {
    struct TestInput;
    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: Char, byte: None, len: 1 }
        }
        fn len(&self) -> usize { 10 }
        fn as_bytes(&self) -> &[u8] { b"abcdefghij" }
    }

    let input = TestInput;
    let result = input.at(11);
}

#[test]
fn test_at_large_value() {
    struct TestInput;
    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: Char, byte: None, len: 1 }
        }
        fn len(&self) -> usize { usize::MAX }
        fn as_bytes(&self) -> &[u8] { b"" }
    }
    
    let input = TestInput;
    let result = input.at(usize::MAX);
}

