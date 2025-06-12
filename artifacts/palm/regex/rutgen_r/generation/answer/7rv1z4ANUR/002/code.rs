// Answer 0

#[test]
fn test_as_byte_not_eof() {
    struct TestStruct(u32);
    
    impl TestStruct {
        fn is_eof(&self) -> bool {
            false
        }

        fn as_byte(&self) -> Option<u8> {
            if self.is_eof() {
                None
            } else {
                Some(self.0 as u8)
            }
        }
    }

    let instance = TestStruct(255); // Max value that can fit in u8
    assert_eq!(instance.as_byte(), Some(255));
}

#[test]
fn test_as_byte_not_eof_zero() {
    struct TestStruct(u32);
    
    impl TestStruct {
        fn is_eof(&self) -> bool {
            false
        }

        fn as_byte(&self) -> Option<u8> {
            if self.is_eof() {
                None
            } else {
                Some(self.0 as u8)
            }
        }
    }

    let instance = TestStruct(0);
    assert_eq!(instance.as_byte(), Some(0));
}

#[test]
fn test_as_byte_not_eof_small_value() {
    struct TestStruct(u32);
    
    impl TestStruct {
        fn is_eof(&self) -> bool {
            false
        }

        fn as_byte(&self) -> Option<u8> {
            if self.is_eof() {
                None
            } else {
                Some(self.0 as u8)
            }
        }
    }

    let instance = TestStruct(100);
    assert_eq!(instance.as_byte(), Some(100));
}

