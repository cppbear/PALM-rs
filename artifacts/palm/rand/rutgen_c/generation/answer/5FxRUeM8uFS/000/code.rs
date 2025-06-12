// Answer 0

#[test]
fn test_d0123() {
    struct MockMach;

    impl Machine for MockMach {
        type u32x4 = [u32; 4];
        type u64x2 = [u64; 2];
        type u64x2x4 = [[u64; 2]; 4];

        fn vec<T: Copy>(&self, lanes: [T; 4]) -> Self::u32x4 {
            [lanes[0] as u32, lanes[1] as u32, lanes[2] as u32, lanes[3] as u32]
        }

        fn unpack<T>(&self, data: T) -> Self::u64x2 {
            // Mocking unpack for testing; returns a fixed pattern for input.
            [1, 2]
        }
    }

    let mach = MockMach;

    let d: vec128_storage = [1, 2]; // Example initialization
    let result = d0123(mach, d);

    assert_eq!(result, [[1, 0], [1, 0], [1, 0], [1, 0]]);
}

#[test]
fn test_d0123_boundary() {
    struct MockMach;

    impl Machine for MockMach {
        type u32x4 = [u32; 4];
        type u64x2 = [u64; 2];
        type u64x2x4 = [[u64; 2]; 4];

        fn vec<T: Copy>(&self, lanes: [T; 4]) -> Self::u32x4 {
            [lanes[0] as u32, lanes[1] as u32, lanes[2] as u32, lanes[3] as u32]
        }

        fn unpack<T>(&self, data: T) -> Self::u64x2 {
            // Example of unpack; you can modify for edge values if necessary
            [u64::MAX, u64::MAX] 
        }
    }

    let mach = MockMach;

    let d: vec128_storage = [0, 0]; // Example of a boundary condition input
    let result = d0123(mach, d);

    assert_eq!(result, [[0, 0], [0, 0], [0, 0], [0, 0]]);
}

