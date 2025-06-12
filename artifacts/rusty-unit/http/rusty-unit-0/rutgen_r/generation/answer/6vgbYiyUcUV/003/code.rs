// Answer 0

#[derive(Debug)]
struct Method(u8);

const HEAD: Method = Method(1);
const GET: Method = Method(2);
const OPTIONS: Method = Method(3);
const TRACE: Method = Method(4);

impl Method {
    pub fn is_safe(&self) -> bool {
        matches!(self.0, 1 | 2 | 3 | 4)
    }
}

#[test]
fn test_is_safe_head() {
    let method = HEAD;
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_get() {
    let method = GET;
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_options() {
    let method = OPTIONS;
    assert!(method.is_safe());
}

#[test]
fn test_is_safe_trace() {
    let method = TRACE;
    assert!(method.is_safe());
}

