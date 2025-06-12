// Answer 0

#[derive(Debug)]
enum N {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}

use std::hash::{Hash, Hasher};

impl Hash for N {
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
fn test_hash_pos_int() {
    use std::collections::hash_map::DefaultHasher;

    let n = N::PosInt(42);
    let mut hasher = DefaultHasher::new();
    n.hash(&mut hasher);
    let result = hasher.finish();
    assert!(result != 0);
}

#[test]
fn test_hash_neg_int() {
    use std::collections::hash_map::DefaultHasher;

    let n = N::NegInt(-42);
    let mut hasher = DefaultHasher::new();
    n.hash(&mut hasher);
    let result = hasher.finish();
    assert!(result != 0);
}

#[test]
fn test_hash_float_zero() {
    use std::collections::hash_map::DefaultHasher;

    let n = N::Float(0.0);
    let mut hasher = DefaultHasher::new();
    n.hash(&mut hasher);
    let result_zero = hasher.finish();

    let n_neg_zero = N::Float(-0.0);
    hasher = DefaultHasher::new();
    n_neg_zero.hash(&mut hasher);
    let result_neg_zero = hasher.finish();

    assert_eq!(result_zero, result_neg_zero);
}

#[test]
fn test_hash_float_non_zero() {
    use std::collections::hash_map::DefaultHasher;

    let n = N::Float(3.14);
    let mut hasher = DefaultHasher::new();
    n.hash(&mut hasher);
    let result = hasher.finish();
    assert!(result != 0);
}

#[test]
fn test_hash_float_neg_non_zero() {
    use std::collections::hash_map::DefaultHasher;

    let n = N::Float(-3.14);
    let mut hasher = DefaultHasher::new();
    n.hash(&mut hasher);
    let result = hasher.finish();
    assert!(result != 0);
}

