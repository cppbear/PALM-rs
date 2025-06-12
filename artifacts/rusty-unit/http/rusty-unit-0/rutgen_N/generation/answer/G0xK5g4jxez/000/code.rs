// Answer 0

#[derive(Debug, PartialEq)]
struct InlineExtension([u8; 10], u8); // Assuming MAX is 10 for the purpose of this test.

#[derive(Debug)]
struct InvalidMethod;

fn write_checked(src: &[u8], data: &mut [u8; 10]) -> Result<(), InvalidMethod> {
    if src.len() > 10 {
        return Err(InvalidMethod);
    }
    data[..src.len()].copy_from_slice(src);
    Ok(())
}

pub fn new(src: &[u8]) -> Result<InlineExtension, InvalidMethod> {
    let mut data: [u8; 10] = Default::default();
    write_checked(src, &mut data)?;
    Ok(InlineExtension(data, src.len() as u8))
}

#[test]
fn test_new_with_valid_input() {
    let src: &[u8] = b"hello";
    let result = new(src);
    assert_eq!(result.is_ok(), true);
    if let Ok(inline_ext) = result {
        assert_eq!(inline_ext.1, 5);
        assert_eq!(&inline_ext.0[..5], b"hello");
    }
}

#[test]
fn test_new_with_empty_input() {
    let src: &[u8] = b"";
    let result = new(src);
    assert_eq!(result.is_ok(), true);
    if let Ok(inline_ext) = result {
        assert_eq!(inline_ext.1, 0);
        assert_eq!(&inline_ext.0[..], b""); // Check for zero-length slice
    }
}

#[test]
fn test_new_with_exceeding_length() {
    let src: &[u8] = b"exceeding length";
    let result = new(src);
    assert_eq!(result.is_ok(), false);
}

