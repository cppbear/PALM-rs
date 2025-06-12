// Answer 0

#[test]
fn test_decoded_len_estimate() {
    struct TestEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }
    
    impl DecodeEstimate for TestEstimate {
        fn decoded_len_estimate(&self) -> usize {
            self.conservative_decoded_len
        }
    }

    let estimate = TestEstimate {
        rem: 2,
        conservative_decoded_len: 10,
    };
    
    assert_eq!(estimate.decoded_len_estimate(), 10);
}

#[test]
fn test_decoded_len_estimate_zero() {
    struct TestEstimate {
        rem: usize,
        conservative_decoded_len: usize,
    }
    
    impl DecodeEstimate for TestEstimate {
        fn decoded_len_estimate(&self) -> usize {
            self.conservative_decoded_len
        }
    }

    let estimate = TestEstimate {
        rem: 0,
        conservative_decoded_len: 0,
    };
    
    assert_eq!(estimate.decoded_len_estimate(), 0);
}

