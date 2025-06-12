// Answer 0

#[derive(Debug)]
pub struct AllocatedExtension(Box<[u8]>);

#[derive(Debug)]
pub struct InvalidMethod;

pub fn write_checked(src: &[u8], dst: &mut Vec<u8>) -> Result<(), InvalidMethod> {
    if src.len() > dst.len() {
        return Err(InvalidMethod);
    }
    dst[..src.len()].copy_from_slice(src);
    Ok(())
}

#[test]
fn test_new_valid_utf8() {
    let src: &[u8] = b"valid_utf8";
    let result = new(src);
    assert!(result.is_ok());
    
    let allocated_extension = result.unwrap();
    assert_eq!(allocated_extension.0, b"valid_utf8".to_vec().into_boxed_slice());
}

#[test]
fn test_new_empty() {
    let src: &[u8] = b"";
    let result = new(src);
    assert!(result.is_ok());
    
    let allocated_extension = result.unwrap();
    assert_eq!(allocated_extension.0.len(), 0);
}

#[test]
#[should_panic]
fn test_new_too_large_src() {
    let src: &[u8] = b"too_large";
    let mut data: Vec<u8> = vec![0; 5];
    
    let _ = write_checked(src, &mut data);
}

#[test]
fn test_new_invalid_utf8() {
    let src: &[u8] = &vec![255, 255, 255]; // invalid UTF-8
    let result = new(src);
    assert!(result.is_err());
}

