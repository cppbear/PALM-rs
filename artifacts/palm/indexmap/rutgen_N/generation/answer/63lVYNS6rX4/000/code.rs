// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct TestSet {
        elements: Vec<i32>,
    }

    impl TestSet {
        fn iter(&self) -> std::slice::Iter<i32> {
            self.elements.iter()
        }
    }

    impl fmt::Debug for TestSet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_set().entries(self.iter()).finish()
        }
    }

    let test_set = TestSet {
        elements: vec![1, 2, 3],
    };
    
    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        test_set.fmt(&mut formatter).unwrap();
    }

    assert_eq!(output, "{1, 2, 3}");
}

