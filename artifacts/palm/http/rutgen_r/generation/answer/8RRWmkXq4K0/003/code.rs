// Answer 0

#[derive(Debug)]
enum Protocol {
    Http,
    Https,
}

#[derive(Debug)]
enum Scheme2 {
    None,
    Standard(Protocol),
    Other(String),
}

struct MyStruct {
    inner: Scheme2,
}

use std::hash::{Hasher, Hash};

impl MyStruct {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        match self.inner {
            Scheme2::None => (),
            Scheme2::Standard(Protocol::Http) => state.write_u8(1),
            Scheme2::Standard(Protocol::Https) => state.write_u8(2),
            Scheme2::Other(ref other) => {
                other.len().hash(state);
                for &b in other.as_bytes() {
                    state.write_u8(b.to_ascii_lowercase());
                }
            }
        }
    }
}

struct TestHasher {
    value: u8,
}

impl Hasher for TestHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.value = bytes.len() as u8;
    }

    fn finish(&self) -> u64 {
        self.value.into()
    }
}

#[test]
fn test_hash_http() {
    let mut hasher = TestHasher { value: 0 };
    let my_struct = MyStruct {
        inner: Scheme2::Standard(Protocol::Http),
    };
    my_struct.hash(&mut hasher);
    assert_eq!(hasher.finish(), 1);
}

#[test]
fn test_hash_https() {
    let mut hasher = TestHasher { value: 0 };
    let my_struct = MyStruct {
        inner: Scheme2::Standard(Protocol::Https),
    };
    my_struct.hash(&mut hasher);
    assert_eq!(hasher.finish(), 2);
}

#[test]
#[should_panic]
fn test_hash_none() {
    let mut hasher = TestHasher { value: 0 };
    let my_struct = MyStruct {
        inner: Scheme2::None,
    };
    my_struct.hash(&mut hasher);
    assert_eq!(hasher.finish(), 0); // This test is supposed to panic.
}

