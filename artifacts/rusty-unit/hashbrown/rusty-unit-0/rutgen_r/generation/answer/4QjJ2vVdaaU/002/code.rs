// Answer 0

#[test]
fn test_move_next_panic() {
    struct TestStruct {
        stride: usize,
        pos: usize,
    }

    impl TestStruct {
        fn move_next(&mut self, bucket_mask: usize) {
            debug_assert!(
                self.stride <= bucket_mask,
                "Went past end of probe sequence"
            );

            self.stride += 8; // Assuming Group::WIDTH is 8.
            self.pos += self.stride;
            self.pos &= bucket_mask;
        }
    }

    let mut instance = TestStruct { stride: 10, pos: 0 };
    let bucket_mask = 5; // This will cause self.stride <= bucket_mask to panic as 10 > 5.

    // This should panic due to the assertion failing.
    let result = std::panic::catch_unwind(|| {
        instance.move_next(bucket_mask);
    });

    assert!(result.is_err());
}

#[test]
fn test_move_next_no_panic() {
    struct TestStruct {
        stride: usize,
        pos: usize,
    }

    impl TestStruct {
        fn move_next(&mut self, bucket_mask: usize) {
            debug_assert!(
                self.stride <= bucket_mask,
                "Went past end of probe sequence"
            );

            self.stride += 8; // Assuming Group::WIDTH is 8.
            self.pos += self.stride;
            self.pos &= bucket_mask;
        }
    }

    let mut instance = TestStruct { stride: 4, pos: 0 };
    let bucket_mask = 10; // This is valid since 4 <= 10.

    // This should not panic.
    let result = std::panic::catch_unwind(|| {
        instance.move_next(bucket_mask);
    });

    assert!(result.is_ok());
    assert_eq!(instance.pos, 4); // After first move_next call with stride incremented.
}

