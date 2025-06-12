// Answer 0

#[test]
fn test_len_with_even_length() {
    struct TestRegex(Vec<char>);
    
    impl TestRegex {
        pub fn len(&self) -> usize {
            self.0.len() / 2
        }
    }

    let regex = TestRegex(vec!['(', 'a', ')', '(', 'b', ')']);
    assert_eq!(regex.len(), 3);
}

#[test]
fn test_len_with_odd_length() {
    struct TestRegex(Vec<char>);
    
    impl TestRegex {
        pub fn len(&self) -> usize {
            self.0.len() / 2
        }
    }

    let regex = TestRegex(vec!['(', 'a', '(', 'b', ')']);
    assert_eq!(regex.len(), 2);
}

#[test]
fn test_len_with_empty_vec() {
    struct TestRegex(Vec<char>);
    
    impl TestRegex {
        pub fn len(&self) -> usize {
            self.0.len() / 2
        }
    }

    let regex = TestRegex(vec![]);
    assert_eq!(regex.len(), 0);
}

#[test]
fn test_len_with_single_group() {
    struct TestRegex(Vec<char>);
    
    impl TestRegex {
        pub fn len(&self) -> usize {
            self.0.len() / 2
        }
    }

    let regex = TestRegex(vec!['(', 'a', ')']);
    assert_eq!(regex.len(), 1);
}

