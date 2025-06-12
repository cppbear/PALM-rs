// Answer 0

#[test]
fn test_hash_zero_float() {
    use core::hash::Hasher;
    use std::collections::hash_map::DefaultHasher;

    #[derive(Copy, Clone)]
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    impl core::hash::Hash for N {
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

    let mut hasher = DefaultHasher::new();
    let number_float_zero = N::Float(0.0);
    number_float_zero.hash(&mut hasher);
    let hash_result_zero = hasher.finish();

    assert_eq!(hash_result_zero, 0u64); // as 0.0f64.to_bits() returns 0
}

#[test]
fn test_hash_negative_zero_float() {
    use core::hash::Hasher;
    use std::collections::hash_map::DefaultHasher;

    #[derive(Copy, Clone)]
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    impl core::hash::Hash for N {
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

    let mut hasher = DefaultHasher::new();
    let number_float_negative_zero = N::Float(-0.0);
    number_float_negative_zero.hash(&mut hasher);
    let hash_result_negative_zero = hasher.finish();

    assert_eq!(hash_result_negative_zero, 0u64); // as -0.0f64.to_bits() also hashes to 0
}

