// Answer 0

#[test]
fn test_find_start_none_due_to_empty_iter() {
    struct TestLit;

    impl TestLit {
        fn iter(&self) -> std::slice::Iter<u8> {
            [].iter()
        }
    }

    let test_struct = TestLit;
    let haystack: &[u8] = b"hello";
    let result = test_struct.find_start(haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_start_none_due_to_lit_longer_than_haystack() {
    struct TestLit {
        lit: Vec<u8>,
    }

    impl TestLit {
        fn iter(&self) -> std::slice::Iter<u8> {
            self.lit.iter()
        }
        
        fn new() -> Self {
            Self { lit: vec![b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0'] }
        }
    }

    let test_struct = TestLit::new();
    let haystack: &[u8] = b"hi";
    let result = test_struct.find_start(haystack);
    assert_eq!(result, None);
}

