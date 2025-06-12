// Answer 0


use std::fmt;

struct Extensions;

impl fmt::Debug for Extensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Extensions").finish()
    }
}

#[test]
fn test_extensions_fmt() {
    let ext = Extensions;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", ext);
    assert!(result.is_ok());
    assert_eq!(buffer, "Extensions");
}

#[test]
fn test_extensions_fmt_empty() {
    let ext = Extensions;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", ext);
    assert!(result.is_ok());
    assert_eq!(buffer, "Extensions");
}


