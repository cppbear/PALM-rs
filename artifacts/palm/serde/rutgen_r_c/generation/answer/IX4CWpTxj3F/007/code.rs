// Answer 0

#[test]
fn test_fmt_map() {
    use std::fmt::{self, Write};
    use crate::Unexpected;

    let unexpected_map = Unexpected::Map;
    let mut output = String::new();
    let result = unexpected_map.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "map");
}

#[test]
fn test_fmt_bytes() {
    use std::fmt::{self, Write};
    use crate::Unexpected;

    let unexpected_bytes = Unexpected::Bytes(&[1, 2, 3]);
    let mut output = String::new();
    let result = unexpected_bytes.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "byte array");
}

#[test]
fn test_fmt_unit() {
    use std::fmt::{self, Write};
    use crate::Unexpected;

    let unexpected_unit = Unexpected::Unit;
    let mut output = String::new();
    let result = unexpected_unit.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "unit value");
}

#[test]
fn test_fmt_other() {
    use std::fmt::{self, Write};
    use crate::Unexpected;

    let unexpected_other = Unexpected::Other("unrecognized input");
    let mut output = String::new();
    let result = unexpected_other.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output, "unrecognized input");
}

