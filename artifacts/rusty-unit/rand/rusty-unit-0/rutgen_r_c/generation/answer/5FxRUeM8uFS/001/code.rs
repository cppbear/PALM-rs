// Answer 0

#[test]
fn test_d0123() {
    struct TestMachine;

    impl Machine for TestMachine {
        type u32x4 = [u32; 4];
        type u64x2 = [u64; 2];
        type u64x2x4 = [[u64; 2]; 4];
        type u32x4x4 = [[u32; 4]; 4];
        
        fn unpack<T>(&self, data: T) -> Self::u64x2 {
            if let Some(array) = data.downcast_ref::<[u64; 2]>() {
                *array
            } else {
                panic!("Unpacking failed");
            }
        }

        fn vec<const N: usize>(&self, values: [u32; N]) -> Self::u32x4 {
            values
        }

        fn from_lanes(values: [[u64; 2]; 4]) -> Self::u64x2x4 {
            values
        }
    }

    let machine = TestMachine;

    // Test input: All values set to zero
    let input_zero: vec128_storage = [0u64, 0u64];
    let result_zero = d0123(machine, input_zero);
    assert_eq!(result_zero, [[0, 0, 0, 0], [1, 0, 0, 0], [2, 0, 0, 0], [3, 0, 0, 0]]);

    // Test input: Max values for u64
    let input_max: vec128_storage = [u64::MAX, u64::MAX];
    let result_max = d0123(machine, input_max);
    assert_eq!(result_max, [[u32::MAX, 0, 0, 0], [1, 0, 0, 0], [2, 0, 0, 0], [3, 0, 0, 0]]);

    // Test input: Arbitrary values
    let input_arbitrary: vec128_storage = [5u64, 10u64];
    let result_arbitrary = d0123(machine, input_arbitrary);
    assert_eq!(result_arbitrary, [[5, 0, 0, 0], [6, 0, 0, 0], [7, 0, 0, 0], [8, 0, 0, 0]]);
}

