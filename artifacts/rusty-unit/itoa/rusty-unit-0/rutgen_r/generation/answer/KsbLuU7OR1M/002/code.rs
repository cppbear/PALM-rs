// Answer 0

#[derive(Debug)]
struct MockInteger;

impl Integer for MockInteger {
    const MAX_STR_LEN: usize = 10; 

    fn write(&self, buffer: &mut <Self as private::Sealed>::Buffer) -> &str {
        let string = "1234567890";
        buffer.copy_from_slice(string.as_bytes());
        std::str::from_utf8(&buffer[..Self::MAX_STR_LEN]).unwrap()
    }
}

#[test]
fn test_format_max_length() {
    let mut buf = [std::mem::MaybeUninit::<u8>::uninit(); 10];
    let result = format(&mut buf, MockInteger);
    assert_eq!(result, "1234567890");
}

#[derive(Debug)]
struct AnotherMockInteger;

impl Integer for AnotherMockInteger {
    const MAX_STR_LEN: usize = 8; 

    fn write(&self, buffer: &mut <Self as private::Sealed>::Buffer) -> &str {
        let string = "12345678";
        buffer.copy_from_slice(string.as_bytes());
        std::str::from_utf8(&buffer[..Self::MAX_STR_LEN]).unwrap()
    }
}

#[test]
fn test_format_exact_length() {
    let mut buf = [std::mem::MaybeUninit::<u8>::uninit(); 8];
    let result = format(&mut buf, AnotherMockInteger);
    assert_eq!(result, "12345678");
}

