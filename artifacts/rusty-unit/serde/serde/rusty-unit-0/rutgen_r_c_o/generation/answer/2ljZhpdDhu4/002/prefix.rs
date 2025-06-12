// Answer 0

#[test]
fn test_write_str_edge_case_exact_fit() {
    let mut buffer = [0u8; 5];
    let mut buf = Buf {
        bytes: &mut buffer,
        offset: 0,
    };
    buf.write_str("Hello").unwrap();
}

#[test]
fn test_write_str_partial_fit() {
    let mut buffer = [0u8; 10];
    let mut buf = Buf {
        bytes: &mut buffer,
        offset: 5,
    };
    buf.write_str("Test").unwrap();
}

#[test]
fn test_write_str_empty_string() {
    let mut buffer = [0u8; 5];
    let mut buf = Buf {
        bytes: &mut buffer,
        offset: 0,
    };
    buf.write_str("").unwrap();
}

#[test]
fn test_write_str_non_empty_with_offset() {
    let mut buffer = [0u8; 10];
    let mut buf = Buf {
        bytes: &mut buffer,
        offset: 2,
    };
    buf.write_str("Hi").unwrap();
}

#[test]
fn test_write_str_max_length() {
    let mut buffer = [0u8; 15];
    let mut buf = Buf {
        bytes: &mut buffer,
        offset: 10,
    };
    buf.write_str("World").unwrap();
}

