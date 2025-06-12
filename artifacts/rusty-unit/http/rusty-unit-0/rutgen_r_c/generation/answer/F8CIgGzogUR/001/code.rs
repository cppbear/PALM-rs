// Answer 0

#[test]
fn test_as_str_valid_utf8() {
    struct InlineExtension([u8; 15], u8);
    impl InlineExtension {
        pub const MAX: usize = 15;
        pub fn new(src: &[u8]) -> Result<InlineExtension, InvalidMethod> {
            if src.len() > Self::MAX {
                return Err(InvalidMethod);
            }
            let mut data = [0u8; Self::MAX];
            let len = src.len() as u8;
            data[..len as usize].copy_from_slice(src);
            Ok(InlineExtension(data, len))
        }
        
        pub fn as_str(&self) -> &str {
            let InlineExtension(ref data, len) = self;
            unsafe { str::from_utf8_unchecked(&data[..*len as usize]) }
        }
    }
    
    let extension = InlineExtension::new(b"Hello, world!").unwrap();
    assert_eq!(extension.as_str(), "Hello, world!");
}

#[test]
#[should_panic]
fn test_as_str_invalid_utf8() {
    struct InlineExtension([u8; 15], u8);
    impl InlineExtension {
        pub const MAX: usize = 15;
        pub fn new(src: &[u8]) -> Result<InlineExtension, InvalidMethod> {
            if src.len() > Self::MAX {
                return Err(InvalidMethod);
            }
            let mut data = [0u8; Self::MAX];
            let len = src.len() as u8;
            data[..len as usize].copy_from_slice(src);
            Ok(InlineExtension(data, len))
        }
        
        pub fn as_str(&self) -> &str {
            let InlineExtension(ref data, len) = self;
            unsafe { str::from_utf8_unchecked(&data[..*len as usize]) }
        }
    }
    
    let extension = InlineExtension::new(b"Hello, \xFF").unwrap();
    extension.as_str();
}

#[test]
fn test_as_str_boundary_length() {
    struct InlineExtension([u8; 15], u8);
    impl InlineExtension {
        pub const MAX: usize = 15;
        pub fn new(src: &[u8]) -> Result<InlineExtension, InvalidMethod> {
            if src.len() > Self::MAX {
                return Err(InvalidMethod);
            }
            let mut data = [0u8; Self::MAX];
            let len = src.len() as u8;
            data[..len as usize].copy_from_slice(src);
            Ok(InlineExtension(data, len))
        }
        
        pub fn as_str(&self) -> &str {
            let InlineExtension(ref data, len) = self;
            unsafe { str::from_utf8_unchecked(&data[..*len as usize]) }
        }
    }
    
    let extension = InlineExtension::new(b"").unwrap();
    assert_eq!(extension.as_str(), "");

    let extension_full = InlineExtension::new(b"Valid UTF-8!").unwrap();
    assert_eq!(extension_full.as_str(), "Valid UTF-8!");
}

