// Answer 0

#[test]
fn test_as_u128_with_positive_integer() {
    struct N {
        value: i64,
    }

    impl N {
        fn as_u128(&self) -> Option<u128> {
            match self.value {
                n if n >= 0 => Some(n as u128),
                _ => None,
            }
        }
    }

    let pos_int_0 = N { value: 0 };
    let pos_int_1 = N { value: 1 };
    let pos_int_100 = N { value: 100 };
    let pos_int_max = N { value: i64::MAX };

    assert_eq!(pos_int_0.as_u128(), Some(0));
    assert_eq!(pos_int_1.as_u128(), Some(1));
    assert_eq!(pos_int_100.as_u128(), Some(100));
    assert_eq!(pos_int_max.as_u128(), Some(u128::MAX)); // i64::MAX fits within u128
}

#[test]
fn test_as_u128_with_negative_integer() {
    struct N {
        value: i64,
    }

    impl N {
        fn as_u128(&self) -> Option<u128> {
            match self.value {
                n if n >= 0 => Some(n as u128),
                _ => None,
            }
        }
    }

    let neg_int_neg1 = N { value: -1 };
    let neg_int_neg100 = N { value: -100 };

    assert_eq!(neg_int_neg1.as_u128(), None);
    assert_eq!(neg_int_neg100.as_u128(), None);
}

#[test]
fn test_as_u128_with_float() {
    struct N {
        value: f64,
    }

    impl N {
        fn as_u128(&self) -> Option<u128> {
            if self.value.fract() == 0.0 && self.value >= 0.0 {
                Some(self.value as u128)
            } else {
                None
            }
        }
    }

    let float_value = N { value: 1.5 };
    let float_neg_value = N { value: -1.5 };

    assert_eq!(float_value.as_u128(), None);
    assert_eq!(float_neg_value.as_u128(), None);
}

