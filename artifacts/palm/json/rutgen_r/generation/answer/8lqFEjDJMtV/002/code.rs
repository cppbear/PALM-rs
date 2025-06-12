// Answer 0

#[derive(Debug)]
enum N {
    PosInt(i32),
    NegInt(i32),
    Float(f64),
}

use std::hash::{Hasher, Hash};

struct TestHasher {
    value: usize,
}

impl Hasher for TestHasher {
    fn finish(&self) -> u64 {
        self.value as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        self.value = bytes.len();
    }
}

impl N {
    fn hash<H: Hasher>(&self, h: &mut H) {
        match *self {
            N::PosInt(i) => i.hash(h),
            N::NegInt(i) => i.hash(h),
            N::Float(f) => {
                if f == 0.0f64 {
                    0.0f64.to_bits().hash(h);
                } else {
                    f.to_bits().hash(h);
                }
            }
        }
    }
}

#[test]
fn test_hash_positive_float() {
    let mut hasher = TestHasher { value: 0 };
    let n = N::Float(3.14);
    n.hash(&mut hasher);
    assert!(hasher.value > 0);
}

#[test]
fn test_hash_negative_float() {
    let mut hasher = TestHasher { value: 0 };
    let n = N::Float(-2.71);
    n.hash(&mut hasher);
    assert!(hasher.value > 0);
}

#[test]
fn test_hash_large_float() {
    let mut hasher = TestHasher { value: 0 };
    let n = N::Float(1e10);
    n.hash(&mut hasher);
    assert!(hasher.value > 0);
}

#[test]
fn test_hash_small_float() {
    let mut hasher = TestHasher { value: 0 };
    let n = N::Float(1e-10);
    n.hash(&mut hasher);
    assert!(hasher.value > 0);
}

