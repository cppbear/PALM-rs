// Answer 0

#[derive(Debug)]
struct Error {
    line: usize,
}

impl Error {
    pub fn new(line: usize) -> Self {
        Self { line }
    }
}

#[derive(Debug)]
struct MyError {
    err: Error,
}

impl MyError {
    pub fn new(line: usize) -> Self {
        Self {
            err: Error::new(line),
        }
    }
}

impl MyError {
    pub fn line(&self) -> usize {
        self.err.line
    }
}

#[test]
fn test_line_zero() {
    let my_error = MyError::new(0);
    assert_eq!(my_error.line(), 0);
}

#[test]
fn test_line_positive() {
    let my_error = MyError::new(5);
    assert_eq!(my_error.line(), 5);
}

#[test]
fn test_line_large_number() {
    let my_error = MyError::new(1000);
    assert_eq!(my_error.line(), 1000);
}

