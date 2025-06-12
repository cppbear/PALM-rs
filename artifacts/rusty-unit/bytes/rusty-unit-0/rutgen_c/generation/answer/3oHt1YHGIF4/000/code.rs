// Answer 0

#[test]
fn test_into_iter_new() {
    struct TestBuffer {
        data: Vec<u8>,
    }

    impl TestBuffer {
        fn as_slice(&self) -> &[u8] {
            &self.data
        }
    }

    let buffer = TestBuffer { data: vec![1, 2, 3] };
    let iter = IntoIter::new(buffer);
    
    assert_eq!(iter.get_ref().as_slice(), &[1, 2, 3]);
}

#[test]
fn test_into_iter_get_ref() {
    struct TestBuffer {
        data: Vec<u8>,
    }

    impl TestBuffer {
        fn as_slice(&self) -> &[u8] {
            &self.data
        }
    }

    let buffer = TestBuffer { data: vec![4, 5, 6] };
    let iter = IntoIter::new(buffer);
    
    assert_eq!(iter.get_ref().as_slice(), &[4, 5, 6]);
}

#[test]
fn test_into_iter_get_mut() {
    struct TestBuffer {
        data: Vec<u8>,
    }

    impl TestBuffer {
        fn as_slice(&self) -> &[u8] {
            &self.data
        }
        fn as_mut_slice(&mut self) -> &mut [u8] {
            &mut self.data
        }
    }

    let mut buffer = TestBuffer { data: vec![7, 8, 9] };
    let mut iter = IntoIter::new(buffer);
    
    assert_eq!(iter.get_mut().as_mut_slice(), &[7, 8, 9]);
    iter.get_mut().data.push(10);
    assert_eq!(iter.get_mut().as_mut_slice(), &[7, 8, 9, 10]);
}

#[test]
fn test_into_iter_into_inner() {
    struct TestBuffer {
        data: Vec<u8>,
    }

    let buffer = TestBuffer { data: vec![11, 12, 13] };
    let iter = IntoIter::new(buffer);
    
    let inner = iter.into_inner();
    assert_eq!(inner.data, vec![11, 12, 13]);
}

