// Answer 0

#[derive(Debug)]
enum N {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}

use std::hash::{Hasher, Hash};

struct TestHasher(u64);

impl Hasher for TestHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = self.0.wrapping_mul(31).wrapping_add(b as u64);
        }
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
fn test_pos_int_hash() {
    let pos_int = N::PosInt(42);
    let mut hasher = TestHasher(0);
    pos_int.hash(&mut hasher);
    assert_eq!(hasher.finish(), 42);
}

#[test]
fn test_another_pos_int_hash() {
    let pos_int = N::PosInt(100);
    let mut hasher = TestHasher(0);
    pos_int.hash(&mut hasher);
    assert_eq!(hasher.finish(), 100);
}

#[test]
fn test_zero_float_hash() {
    let zero_float = N::Float(0.0);
    let mut hasher = TestHasher(0);
    zero_float.hash(&mut hasher);
    assert_eq!(hasher.finish(), 0);
} 

#[test]
fn test_neg_float_hash() {
    let neg_float = N::Float(-1.5);
    let mut hasher = TestHasher(0);
    neg_float.hash(&mut hasher);
    assert_eq!(hasher.finish(), (-1.5f64.to_bits()));
} 

#[test]
fn test_pos_float_hash() {
    let pos_float = N::Float(3.14);
    let mut hasher = TestHasher(0);
    pos_float.hash(&mut hasher);
    assert_eq!(hasher.finish(), (3.14f64.to_bits()));
} 

