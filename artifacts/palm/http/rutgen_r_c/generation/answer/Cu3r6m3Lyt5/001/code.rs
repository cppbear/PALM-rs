// Answer 0

#[test]
fn test_fmt_empty_data() {
    use bytes::Bytes;
    
    struct TestByteStr {
        bytes: Bytes,
    }
    
    impl TestByteStr {
        fn is_empty(&self) -> bool {
            self.bytes.is_empty()
        }
        
        fn as_bytes(&self) -> &[u8] {
            &self.bytes
        }
    }

    let empty_bytes = Bytes::from_static(b"");
    let empty_data = TestByteStr { bytes: empty_bytes };
    
    let path_and_query = PathAndQuery {
        data: empty_data,
        query: NONE,
    };

    let mut output = String::new();
    let result = path_and_query.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "/");
}

#[test]
fn test_fmt_data_starts_with_slash() {
    use bytes::Bytes;

    struct TestByteStr {
        bytes: Bytes,
    }
    
    impl TestByteStr {
        fn is_empty(&self) -> bool {
            self.bytes.is_empty()
        }
        
        fn as_bytes(&self) -> &[u8] {
            &self.bytes
        }
    }

    let slash_bytes = Bytes::from_static(b"/path");
    let data_with_slash = TestByteStr { bytes: slash_bytes };
    
    let path_and_query = PathAndQuery {
        data: data_with_slash,
        query: NONE,
    };

    let mut output = String::new();
    let result = path_and_query.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "/path");
}

#[test]
fn test_fmt_data_starts_with_asterisk() {
    use bytes::Bytes;

    struct TestByteStr {
        bytes: Bytes,
    }
    
    impl TestByteStr {
        fn is_empty(&self) -> bool {
            self.bytes.is_empty()
        }
        
        fn as_bytes(&self) -> &[u8] {
            &self.bytes
        }
    }

    let asterisk_bytes = Bytes::from_static(b"*/data");
    let data_with_asterisk = TestByteStr { bytes: asterisk_bytes };
    
    let path_and_query = PathAndQuery {
        data: data_with_asterisk,
        query: NONE,
    };

    let mut output = String::new();
    let result = path_and_query.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "*/data");
}

#[test]
fn test_fmt_data_starts_with_other_character() {
    use bytes::Bytes;

    struct TestByteStr {
        bytes: Bytes,
    }
    
    impl TestByteStr {
        fn is_empty(&self) -> bool {
            self.bytes.is_empty()
        }
        
        fn as_bytes(&self) -> &[u8] {
            &self.bytes
        }
    }

    let other_bytes = Bytes::from_static(b"hello");
    let data_with_other_char = TestByteStr { bytes: other_bytes };
    
    let path_and_query = PathAndQuery {
        data: data_with_other_char,
        query: NONE,
    };

    let mut output = String::new();
    let result = path_and_query.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "/hello");
}

