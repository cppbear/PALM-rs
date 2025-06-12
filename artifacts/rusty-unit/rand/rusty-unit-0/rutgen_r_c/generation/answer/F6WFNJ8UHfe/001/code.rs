// Answer 0

#[test]
fn test_diagonalize() {
    struct TestVector(u32, u32, u32, u32);

    impl LaneWords4 for TestVector {
        fn shuffle_lane_words3012(self) -> Self {
            TestVector(self.1, self.2, self.0, self.3)
        }

        fn shuffle_lane_words2301(self) -> Self {
            TestVector(self.2, self.0, self.1, self.3)
        }

        fn shuffle_lane_words1230(self) -> Self {
            TestVector(self.1, self.2, self.3, self.0)
        }
    }

    let input_state = State {
        a: TestVector(1, 2, 3, 4),
        b: TestVector(5, 6, 7, 8),
        c: TestVector(9, 10, 11, 12),
        d: TestVector(13, 14, 15, 16),
    };

    let result = diagonalize(input_state);

    assert_eq!(result.b, TestVector(6, 7, 5, 8)); // Expected from shuffle_lane_words3012
    assert_eq!(result.c, TestVector(10, 9, 1, 12)); // Expected from shuffle_lane_words2301
    assert_eq!(result.d, TestVector(7, 8, 9, 13)); // Expected from shuffle_lane_words1230
}

