// Answer 0

#[test]
fn test_len() {
    struct TestRegex {
        locs: Vec<usize>,
    }
    
    impl TestRegex {
        fn new(locs: Vec<usize>) -> Self {
            TestRegex { locs }
        }
        
        pub fn len(&self) -> usize {
            self.locs.len()
        }
    }

    let regex_with_one_capture = TestRegex::new(vec![0]);
    assert_eq!(regex_with_one_capture.len(), 1);

    let regex_with_multiple_captures = TestRegex::new(vec![0, 1, 2]);
    assert_eq!(regex_with_multiple_captures.len(), 3);

    let regex_with_no_captures = TestRegex::new(Vec::new());
    assert_eq!(regex_with_no_captures.len(), 0);
}

