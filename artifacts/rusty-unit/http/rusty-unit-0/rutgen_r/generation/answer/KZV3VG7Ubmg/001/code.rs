// Answer 0

#[derive(Debug)]
struct HeaderValue {
    value: Vec<u8>,
}

impl HeaderValue {
    pub fn from_shared(src: Bytes) -> Result<Self, InvalidHeaderValue> {
        Ok(HeaderValue { value: src.to_vec() })
    }

    pub fn from_bytes(src: &[u8]) -> Result<Self, InvalidHeaderValue> {
        Ok(HeaderValue { value: src.to_vec() })
    }
}

struct InvalidHeaderValue;

#[derive(Debug)]
struct Bytes(Vec<u8>);

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

pub fn from_maybe_shared<T>(src: T) -> Result<HeaderValue, InvalidHeaderValue>
where
    T: AsRef<[u8]> + 'static,
{
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<Bytes>() {
        return HeaderValue::from_shared(src);
    }

    HeaderValue::from_bytes(src.as_ref())
}

#[test]
fn test_from_maybe_shared_with_bytes() {
    let bytes = Bytes(vec![1, 2, 3]);
    let result = from_maybe_shared(bytes);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, vec![1, 2, 3]);
}

#[test]
fn test_from_maybe_shared_with_slice() {
    let slice: &[u8] = &[4, 5, 6];
    let result = from_maybe_shared(slice);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, vec![4, 5, 6]);
}

#[test]
fn test_from_maybe_shared_with_empty_slice() {
    let slice: &[u8] = &[];
    let result = from_maybe_shared(slice);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, vec![]);
}

