// Answer 0

#[derive(Debug)]
struct InvalidMethod;

impl InvalidMethod {
    fn new() -> Self {
        InvalidMethod
    }
}

const METHOD_CHARS: [u8; 256] = [0; 256]; // Initialized with zeros for simplicity

fn write_checked(src: &[u8], dst: &mut [u8]) -> Result<(), InvalidMethod> {
    for (i, &b) in src.iter().enumerate() {
        let b = METHOD_CHARS[b as usize];

        if b == 0 {
            return Err(InvalidMethod::new());
        }

        dst[i] = b;
    }

    Ok(())
}

#[test]
fn test_write_checked_with_non_zero_character() {
    let mut dst = [0u8; 5];
    let src = [1, 2, 3, 4, 5]; // Assuming 1 to 5 will map to non-zero values in METHOD_CHARS
    for (i, &b) in src.iter().enumerate() {
        METHOD_CHARS[b as usize] = b; // Setting some mapping to non-zero values
    }
    let result = write_checked(&src, &mut dst);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_write_checked_with_zero_character() {
    let mut dst = [0u8; 5];
    let src = [0]; // Ensure index 0 in METHOD_CHARS is zero
    let result = write_checked(&src, &mut dst);
    assert_eq!(result, Err(InvalidMethod::new()));
}

#[test]
fn test_write_checked_with_boundary_values() {
    let mut dst = [0u8; 5];
    let src = [255]; // Edge case, if METHOD_CHARS[255] happens to be zero
    let result = write_checked(&src, &mut dst);
    assert_eq!(result, Err(InvalidMethod::new()));
}

