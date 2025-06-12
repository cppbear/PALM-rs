// Answer 0

#[derive(Debug)]
struct HeaderValue {
    inner: Bytes,
    is_sensitive: bool,
}

impl HeaderValue {
    fn from_maybe_shared<T>(src: T) -> Result<Self, &'static str>
    where
        T: AsRef<[u8]> + 'static,
    {
        // Dummy implementation for testing purpose
        if std::str::from_utf8(src.as_ref()).is_ok() {
            Ok(HeaderValue {
                inner: Bytes::copy_from_slice(src.as_ref()),
                is_sensitive: false,
            })
        } else {
            Err("Invalid UTF-8")
        }
    }
}

#[derive(Debug)]
struct Bytes(Vec<u8>);

impl Bytes {
    fn copy_from_slice(slice: &[u8]) -> Self {
        Bytes(slice.to_vec())
    }
    
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[test]
fn test_from_maybe_shared_unchecked_valid_utf8() {
    let valid_bytes = Bytes(vec![72, 101, 108, 108, 111]); // Hello in UTF-8
    let header_value = unsafe { from_maybe_shared_unchecked(valid_bytes) };
    
    assert_eq!(header_value.inner.as_ref(), b"Hello");
}

#[test]
#[should_panic(expected = "HeaderValue::from_maybe_shared_unchecked() with invalid bytes")]
fn test_from_maybe_shared_unchecked_invalid_utf8() {
    let invalid_bytes = Bytes(vec![0, 159, 146, 150]); // Invalid UTF-8 sequence
    unsafe { from_maybe_shared_unchecked(invalid_bytes) };
}

#[test]
fn test_from_maybe_shared_unchecked_empty() {
    let empty_bytes = Bytes(vec![]);
    let header_value = unsafe { from_maybe_shared_unchecked(empty_bytes) };
    
    assert_eq!(header_value.inner.as_ref(), b"");
}

