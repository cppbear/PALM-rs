// Answer 0

#[derive(Debug)]
enum N {
    PosInt(i32),
    NegInt(i32),
    Float(f64),
}

use std::hash::{Hasher, Hash};

struct TestHasher(u64);

impl Hasher for TestHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 = self.0.wrapping_mul(31).wrapping_add(*byte as u64);
        }
    }

    fn write_u64(&mut self, i: u64) {
        self.write(&i.to_le_bytes());
    }
}

#[test]
fn test_hash_float_zero() {
    let float_value = N::Float(0.0);
    let mut hasher = TestHasher(0);
    float_value.hash(&mut hasher);
    assert_eq!(hasher.finish(), TestHasher(0).finish());
}

#[test]
fn test_hash_float_negative_zero() {
    let float_value = N::Float(-0.0);
    let mut hasher = TestHasher(0);
    float_value.hash(&mut hasher);
    assert_eq!(hasher.finish(), TestHasher(0).finish());
}

