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
        self.value += bytes.len();
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
fn test_neg_int_hash() {
    let neg_one = N::NegInt(-1);
    let neg_two = N::NegInt(-2);
    
    let mut hasher_one = TestHasher { value: 0 };
    neg_one.hash(&mut hasher_one);
    
    let mut hasher_two = TestHasher { value: 0 };
    neg_two.hash(&mut hasher_two);
    
    assert!(hasher_one.value != hasher_two.value);
}

#[test]
fn test_neg_int_zero_hash() {
    let neg_zero = N::NegInt(0); // Though it's zero, it's treated as Negative Int in this case
    
    let mut hasher = TestHasher { value: 0 };
    neg_zero.hash(&mut hasher);
    
    assert!(hasher.value > 0); // Check that it's been hashed (non-zero length)
}

