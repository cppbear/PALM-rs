// Answer 0

#[derive(Debug)]
struct Error {
    column: usize,
}

struct Context {
    err: Error,
}

impl Context {
    fn new(column: usize) -> Self {
        Context {
            err: Error { column },
        }
    }
}

#[test]
fn test_column_zero() {
    let ctx = Context::new(0);
    assert_eq!(ctx.err.column, 0);
}

#[test]
fn test_column_one() {
    let ctx = Context::new(1);
    assert_eq!(ctx.err.column, 1);
}

#[test]
fn test_column_boundary() {
    let ctx = Context::new(usize::MAX);
    assert_eq!(ctx.err.column, usize::MAX);
}

#[test]
fn test_column_non_zero() {
    let ctx = Context::new(5);
    assert_eq!(ctx.err.column, 5);
}

