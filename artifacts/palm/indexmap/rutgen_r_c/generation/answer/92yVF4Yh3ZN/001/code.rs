// Answer 0

#[test]
fn test_split_last_non_empty_slice() {
    struct TestSlice {
        entries: [Bucket<i32>; 3],
    }

    impl TestSlice {
        fn new() -> Self {
            Self {
                entries: [
                    Bucket { hash: 0, key: 1, value: 10 },
                    Bucket { hash: 0, key: 2, value: 20 },
                    Bucket { hash: 0, key: 3, value: 30 },
                ],
            }
        }
        
        fn as_slice(&self) -> &Slice<i32> {
            Slice::from_slice(&self.entries)
        }
    }

    let test_slice = TestSlice::new();
    let slice = test_slice.as_slice();
    let result = slice.split_last();
    
    assert!(result.is_some());
    let (last_key, rest) = result.unwrap();
    
    assert_eq!(*last_key, 3);
    assert_eq!(rest.len(), 2);
    assert!(rest.first().is_some());
    assert_eq!(rest.first().unwrap().key, 1);
    assert_eq!(rest.last().unwrap().key, 2);
}

#[test]
fn test_split_last_empty_slice() {
    struct EmptySlice {
        entries: [Bucket<i32>; 0],
    }

    impl EmptySlice {
        fn as_slice(&self) -> &Slice<i32> {
            Slice::from_slice(&self.entries)
        }
    }

    let empty_slice = EmptySlice { entries: [] };
    let slice = empty_slice.as_slice();
    let result = slice.split_last();
    
    assert!(result.is_none());
}

