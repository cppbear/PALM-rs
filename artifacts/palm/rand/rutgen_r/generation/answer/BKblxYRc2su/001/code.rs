// Answer 0

#[test]
fn test_set_nonce_valid_value() {
    struct MockChacha {
        nonce: u64,
    }

    impl MockChacha {
        pub fn set_nonce(&mut self, value: u64) {
            self.nonce = value;
        }
    }

    let mut chacha = MockChacha { nonce: 0 };
    chacha.set_nonce(12345);
    assert_eq!(chacha.nonce, 12345);
}

#[test]
fn test_set_nonce_zero_value() {
    struct MockChacha {
        nonce: u64,
    }

    impl MockChacha {
        pub fn set_nonce(&mut self, value: u64) {
            self.nonce = value;
        }
    }

    let mut chacha = MockChacha { nonce: 0 };
    chacha.set_nonce(0);
    assert_eq!(chacha.nonce, 0);
}

#[should_panic]
#[test]
fn test_set_nonce_exceeding_value() {
    struct MockChacha {
        nonce: u64,
    }

    impl MockChacha {
        pub fn set_nonce(&mut self, value: u64) {
            if value > u64::MAX {
                panic!("Value exceeds maximum u64");
            }
            self.nonce = value;
        }
    }

    let mut chacha = MockChacha { nonce: 0 };
    chacha.set_nonce(u64::MAX + 1);
}

