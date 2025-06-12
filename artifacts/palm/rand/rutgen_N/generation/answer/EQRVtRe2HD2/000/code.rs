// Answer 0

#[test]
fn test_add_pos() {
    struct TestMachine;

    impl Machine for TestMachine {
        type u32x4 = [u32; 4];
        type u64x2 = [u64; 2];

        fn unpack(self, value: u64) -> Self::u64x2 {
            [value as u64, (value >> 32) as u64]
        }
        
        fn vec(self, arr: [u64; 2]) -> u64 {
            arr[0] | (arr[1] << 32)
        }
    }

    let machine = TestMachine;
    let d: TestMachine::u32x4 = [1, 2, 3, 4];
    let i: u64 = 10;

    let result = add_pos(machine, d, i);
    assert_eq!(result, [11, 2, 3, 4]); // Ensure the increment was applied correctly
}

#[test]
fn test_add_pos_boundary() {
    struct TestMachine;

    impl Machine for TestMachine {
        type u32x4 = [u32; 4];
        type u64x2 = [u64; 2];

        fn unpack(self, value: u64) -> Self::u64x2 {
            [value as u64, (value >> 32) as u64]
        }
        
        fn vec(self, arr: [u64; 2]) -> u64 {
            arr[0] | (arr[1] << 32)
        }
    }

    let machine = TestMachine;
    let d: TestMachine::u32x4 = [u32::MAX, u32::MAX, u32::MAX, u32::MAX]; // Maximum values
    let i: u64 = 1;

    let result = add_pos(machine, d, i);
    assert_eq!(result, [0, u32::MAX, u32::MAX, u32::MAX]); // Check wraparound behavior for the first element
}

