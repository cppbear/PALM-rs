// Answer 0

#[test]
fn test_hash_float_zero() {
    use std::hash::{Hash, Hasher, SipHasher};

    enum N {
        PosInt(i32),
        NegInt(i32),
        Float(f64),
    }

    let f = N::Float(0.0f64);
    let mut hasher = SipHasher::new();
    f.hash(&mut hasher);
}

#[test]
fn test_hash_float_neg_zero() {
    use std::hash::{Hash, Hasher, SipHasher};

    enum N {
        PosInt(i32),
        NegInt(i32),
        Float(f64),
    }

    let f = N::Float(-0.0f64);
    let mut hasher = SipHasher::new();
    f.hash(&mut hasher);
}

