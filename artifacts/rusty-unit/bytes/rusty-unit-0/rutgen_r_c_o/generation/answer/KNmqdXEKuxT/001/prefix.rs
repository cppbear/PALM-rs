// Answer 0

#[test]
fn test_last_ref_non_empty() {
    let chain = Chain::new(&b"hello"[..], &b"world"[..]);
    let result = chain.last_ref();
}

#[test]
fn test_last_ref_empty_first() {
    let chain = Chain::new(&b""[..], &b"world"[..]);
    let result = chain.last_ref();
}

#[test]
fn test_last_ref_empty_both() {
    let chain = Chain::new(&b""[..], &b""[..]);
    let result = chain.last_ref();
}

#[test]
fn test_last_ref_small_first() {
    let chain = Chain::new(&b"a"[..], &b"world"[..]);
    let result = chain.last_ref();
}

#[test]
fn test_last_ref_large_first() {
    let chain = Chain::new(&b"hello world!"[..], &b"world"[..]);
    let result = chain.last_ref();
}

