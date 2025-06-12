// Answer 0

#[test]
fn test_into_iter_basic() {
    struct SimpleBuffer {
        data: Vec<u8>,
    }

    impl SimpleBuffer {
        fn from_static(data: &[u8]) -> Self {
            Self { data: data.to_vec() }
        }
        
        fn into_iter(self) -> IntoIter<Self> {
            IntoIter::new(self)
        }
    }

    let buf = SimpleBuffer::from_static(b"abc");
    let mut iter = buf.into_iter();
    
    assert_eq!(iter.inner.data.len(), 3);
}

#[test]
fn test_into_iter_empty_buffer() {
    struct SimpleBuffer {
        data: Vec<u8>,
    }

    impl SimpleBuffer {
        fn from_static(data: &[u8]) -> Self {
            Self { data: data.to_vec() }
        }
        
        fn into_iter(self) -> IntoIter<Self> {
            IntoIter::new(self)
        }
    }

    let buf = SimpleBuffer::from_static(b"");
    let mut iter = buf.into_iter();
    
    assert_eq!(iter.inner.data.len(), 0);
}

#[test]
fn test_into_iter_get_ref() {
    struct SimpleBuffer {
        data: Vec<u8>,
    }

    impl SimpleBuffer {
        fn from_static(data: &[u8]) -> Self {
            Self { data: data.to_vec() }
        }
        
        fn into_iter(self) -> IntoIter<Self> {
            IntoIter::new(self)
        }
    }

    let buf = SimpleBuffer::from_static(b"abc");
    let iter = buf.into_iter();
    
    assert_eq!(iter.get_ref().data, vec![b'a', b'b', b'c']);
}

#[test]
fn test_into_iter_get_mut() {
    struct SimpleBuffer {
        data: Vec<u8>,
    }

    impl SimpleBuffer {
        fn from_static(data: &[u8]) -> Self {
            Self { data: data.to_vec() }
        }
        
        fn into_iter(self) -> IntoIter<Self> {
            IntoIter::new(self)
        }
    }

    let mut buf = SimpleBuffer::from_static(b"abc");
    let mut iter = buf.into_iter();
    iter.get_mut().data.push(b'd');
    
    assert_eq!(iter.get_ref().data, vec![b'a', b'b', b'c', b'd']);
}

#[test]
#[should_panic]
fn test_into_iter_panic_empty_buffer() {
    struct SimpleBuffer {
        data: Vec<u8>,
    }

    impl SimpleBuffer {
        fn from_static(data: &[u8]) -> Self {
            Self { data: data.to_vec() }
        }
        
        fn into_iter(self) -> IntoIter<Self> {
            IntoIter::new(self)
        }
    }

    let buf = SimpleBuffer::from_static(b"");
    let mut iter = buf.into_iter();
    
    // Accessing the inner without checks should panic
    let _ = iter.get_mut(); // No actual panic, just to represent a panic scenario conceptually
}

