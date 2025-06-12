// Answer 0

#[derive(Debug)]
struct Class {
    inner: InnerClass,
}

#[derive(Debug)]
enum InnerClass {
    Unicode(),
    Bytes(BytesClass),
}

#[derive(Debug)]
struct BytesClass {
    is_all_ascii: bool,
}

impl BytesClass {
    fn is_all_ascii(&self) -> bool {
        self.is_all_ascii
    }
}

impl Class {
    pub fn is_always_utf8(&self) -> bool {
        match &self.inner {
            InnerClass::Unicode(_) => true,
            InnerClass::Bytes(x) => x.is_all_ascii(),
        }
    }
}

#[test]
fn test_is_always_utf8_all_ascii() {
    let bytes_class = BytesClass { is_all_ascii: true };
    let class = Class { inner: InnerClass::Bytes(bytes_class) };
    assert!(class.is_always_utf8());
}

#[test]
fn test_is_always_utf8_not_all_ascii() {
    let bytes_class = BytesClass { is_all_ascii: false };
    let class = Class { inner: InnerClass::Bytes(bytes_class) };
    assert!(!class.is_always_utf8());
}

#[test]
fn test_is_always_utf8_unicode() {
    let class = Class { inner: InnerClass::Unicode() };
    assert!(class.is_always_utf8());
}

