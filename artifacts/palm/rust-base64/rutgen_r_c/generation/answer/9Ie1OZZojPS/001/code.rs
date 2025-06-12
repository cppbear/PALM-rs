// Answer 0

#[test]
fn test_decoded_len_estimate_zero() {
    struct TestEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let estimate = TestEstimate {
        rem: 0,
        conservative_decoded_len: 0,
    };
    assert_eq!(estimate.decoded_len_estimate(), 0);
}

#[test]
fn test_decoded_len_estimate_positive() {
    struct TestEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let estimate = TestEstimate {
        rem: 1,
        conservative_decoded_len: 100,
    };
    assert_eq!(estimate.decoded_len_estimate(), 100);
}

#[test]
fn test_decoded_len_estimate_large_value() {
    struct TestEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }

    let estimate = TestEstimate {
        rem: 3,
        conservative_decoded_len: 1_000_000,
    };
    assert_eq!(estimate.decoded_len_estimate(), 1_000_000);
}

