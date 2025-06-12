// Answer 0

#[test]
fn test_len_non_empty_slice() {
    struct TestSlice<'a>(&'a [i32]);
    
    impl<'a> Index<usize> for TestSlice<'a> {
        type Output = i32;

        fn index(&self, index: usize) -> &Self::Output {
            &self.0[index]
        }
    }
    
    impl<'a> IndexedRandom for TestSlice<'a> {
        fn len(&self) -> usize {
            self.0.len()
        }
    }

    let slice = TestSlice(&[1, 2, 3, 4, 5]);
    assert_eq!(slice.len(), 5);
}

#[test]
fn test_len_empty_slice() {
    struct TestSlice<'a>(&'a [i32]);
    
    impl<'a> Index<usize> for TestSlice<'a> {
        type Output = i32;

        fn index(&self, index: usize) -> &Self::Output {
            &self.0[index]
        }
    }
    
    impl<'a> IndexedRandom for TestSlice<'a> {
        fn len(&self) -> usize {
            self.0.len()
        }
    }

    let slice = TestSlice(&[]);
    assert_eq!(slice.len(), 0);
}

