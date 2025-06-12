// Answer 0

#[derive(Debug)]
struct Splice<'a, T> {
    drain: &'a [T],
    replace_with: &'a [T],
}

impl<'a, T: std::fmt::Debug> Splice<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Splice")
            .field("drain", &self.drain)
            .field("replace_with", &self.replace_with)
            .finish()
    }
}

#[test]
fn test_fmt_with_non_empty_arrays() {
    let drain = &[1, 2, 3];
    let replace_with = &[4, 5, 6];
    let splice = Splice { drain, replace_with };
    
    let mut output = std::fmt::Formatter::new();
    let result = splice.fmt(&mut output);
    
    assert!(result.is_ok());
}

#[test]
fn test_fmt_with_empty_drain() {
    let drain: Vec<i32> = Vec::new();
    let replace_with = &[1, 2, 3];
    let splice = Splice {
        drain: &drain,
        replace_with,
    };
    
    let mut output = std::fmt::Formatter::new();
    let result = splice.fmt(&mut output);
    
    assert!(result.is_ok());
}

#[test]
fn test_fmt_with_empty_replace_with() {
    let drain = &[1, 2, 3];
    let replace_with: Vec<i32> = Vec::new();
    let splice = Splice {
        drain,
        replace_with: &replace_with,
    };
    
    let mut output = std::fmt::Formatter::new();
    let result = splice.fmt(&mut output);
    
    assert!(result.is_ok());
}

#[test]
fn test_fmt_with_both_empty() {
    let drain: Vec<i32> = Vec::new();
    let replace_with: Vec<i32> = Vec::new();
    let splice = Splice {
        drain: &drain,
        replace_with: &replace_with,
    };
    
    let mut output = std::fmt::Formatter::new();
    let result = splice.fmt(&mut output);
    
    assert!(result.is_ok());
}

#[should_panic]
fn test_fmt_with_uninitialized_formatter() {
    let drain = &[1, 2, 3];
    let replace_with = &[4, 5, 6];
    let splice = Splice { drain, replace_with };
    
    let result = splice.fmt(std::ptr::null_mut());
    assert!(result.is_err());
}

