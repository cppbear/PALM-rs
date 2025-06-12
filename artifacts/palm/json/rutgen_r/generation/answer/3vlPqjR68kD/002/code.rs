// Answer 0

#[test]
fn test_size_hint_exact_bounds() {
    struct IterExact {
        data: Vec<i32>,
        index: usize,
    }

    impl IterExact {
        fn new(data: Vec<i32>) -> Self {
            Self { data, index: 0 }
        }
        
        fn size_hint(&self) -> (usize, Option<usize>) {
            let len = self.data.len();
            (len, Some(len))
        }
        
        fn next(&mut self) -> Option<i32> {
            if self.index < self.data.len() {
                let result = Some(self.data[self.index]);
                self.index += 1;
                result
            } else {
                None
            }
        }
    }

    struct TestStruct {
        iter: IterExact,
    }
    
    impl TestStruct {
        fn new(data: Vec<i32>) -> Self {
            Self { iter: IterExact::new(data) }
        }
        
        fn size_hint(&self) -> Option<usize> {
            match self.iter.size_hint() {
                (lower, Some(upper)) if lower == upper => Some(upper),
                _ => None,
            }
        }
    }

    let data = vec![1, 2, 3];
    let test_instance = TestStruct::new(data);
    
    assert_eq!(test_instance.size_hint(), Some(3));
}

