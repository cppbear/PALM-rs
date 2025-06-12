// Answer 0

#[test]
fn test_diagonalize() {
    struct TestVec([u32; 4]);

    impl LaneWords4 for TestVec {
        fn shuffle_lane_words3012(self) -> Self {
            let mut array = self.0;
            array.swap(1, 0);
            array.swap(2, 3);
            TestVec(array)
        }

        fn shuffle_lane_words2301(self) -> Self {
            let mut array = self.0;
            array.swap(2, 1);
            array.swap(0, 3);
            TestVec(array)
        }

        fn shuffle_lane_words1230(self) -> Self {
            let mut array = self.0;
            array.swap(1, 0);
            array.swap(3, 2);
            TestVec(array)
        }
    }

    let state = State {
        a: TestVec([1, 2, 3, 4]),
        b: TestVec([5, 6, 7, 8]),
        c: TestVec([9, 10, 11, 12]),
        d: TestVec([13, 14, 15, 16]),
    };

    let result = diagonalize(state);

    assert_eq!(result.b.0, [6, 5, 12, 11]);
    assert_eq!(result.c.0, [10, 9, 14, 13]);
    assert_eq!(result.d.0, [4, 3, 15, 16]);
}

#[test]
fn test_diagonalize_boundary_conditions() {
    struct TestVecBoundary([u32; 4]);

    impl LaneWords4 for TestVecBoundary {
        fn shuffle_lane_words3012(self) -> Self {
            let mut array = self.0;
            array.swap(1, 0);
            array.swap(2, 3);
            TestVecBoundary(array)
        }

        fn shuffle_lane_words2301(self) -> Self {
            let mut array = self.0;
            array.swap(2, 1);
            array.swap(0, 3);
            TestVecBoundary(array)
        }

        fn shuffle_lane_words1230(self) -> Self {
            let mut array = self.0;
            array.swap(1, 0);
            array.swap(3, 2);
            TestVecBoundary(array)
        }
    }

    let state_min = State {
        a: TestVecBoundary([0, 0, 0, 0]),
        b: TestVecBoundary([0, 0, 0, 0]),
        c: TestVecBoundary([0, 0, 0, 0]),
        d: TestVecBoundary([0, 0, 0, 0]),
    };

    let result_min = diagonalize(state_min);

    assert_eq!(result_min.b.0, [0, 0, 0, 0]);
    assert_eq!(result_min.c.0, [0, 0, 0, 0]);
    assert_eq!(result_min.d.0, [0, 0, 0, 0]);

    let state_max = State {
        a: TestVecBoundary([u32::MAX, u32::MAX, u32::MAX, u32::MAX]),
        b: TestVecBoundary([u32::MAX, u32::MAX, u32::MAX, u32::MAX]),
        c: TestVecBoundary([u32::MAX, u32::MAX, u32::MAX, u32::MAX]),
        d: TestVecBoundary([u32::MAX, u32::MAX, u32::MAX, u32::MAX]),
    };

    let result_max = diagonalize(state_max);

    assert_eq!(result_max.b.0, [u32::MAX, u32::MAX, u32::MAX, u32::MAX]);
    assert_eq!(result_max.c.0, [u32::MAX, u32::MAX, u32::MAX, u32::MAX]);
    assert_eq!(result_max.d.0, [u32::MAX, u32::MAX, u32::MAX, u32::MAX]);
}

