// Answer 0

#[test]
fn test_deref_returns_slice() {
    struct ByteWrapper {
        data: Vec<u8>,
    }

    impl ByteWrapper {
        fn as_slice(&self) -> &[u8] {
            &self.data
        }

        fn new(data: Vec<u8>) -> Self {
            ByteWrapper { data }
        }
    }

    let wrapper = ByteWrapper::new(vec![1, 2, 3, 4]);

    let result: &[u8] = wrapper.deref();
    
    assert_eq!(result, &[1, 2, 3, 4]);
}

#[test]
fn test_deref_empty_slice() {
    struct ByteWrapper {
        data: Vec<u8>,
    }

    impl ByteWrapper {
        fn as_slice(&self) -> &[u8] {
            &self.data
        }

        fn new(data: Vec<u8>) -> Self {
            ByteWrapper { data }
        }
    }

    let wrapper = ByteWrapper::new(vec![]);

    let result: &[u8] = wrapper.deref();
    
    assert_eq!(result, &[]);
}

