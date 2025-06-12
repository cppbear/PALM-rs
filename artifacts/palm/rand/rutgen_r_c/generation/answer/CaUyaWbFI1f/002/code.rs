// Answer 0

#[test]
fn test_refill_wide_impl_zero_rounds() {
    struct FakeMach;

    impl Machine for FakeMach {
        type u32x4 = [u32; 4];
        type u32x4x4 = [[u32; 4]; 4];
        type u64x2 = [u64; 2];
        type u64x2x4 = [[u64; 2]; 4];

        fn vec(&self, arr: [u32; 4]) -> Self::u32x4 {
            arr
        }

        fn unpack(&self, val: vec128_storage) -> Self::u32x4 {
            val.0
        }

        fn read_le(&self, slice: &[u8]) -> Self::u32x4 {
            let arr = [
                u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]),
                u32::from_le_bytes([slice[4], slice[5], slice[6], slice[7]]),
                u32::from_le_bytes([slice[8], slice[9], slice[10], slice[11]]),
                u32::from_le_bytes([slice[12], slice[13], slice[14], slice[15]]),
            ];
            arr
        }
    }

    let mut state = ChaCha {
        b: vec128_storage([0; 4]),
        c: vec128_storage([0; 4]),
        d: vec128_storage([0; 4]),
    };
    let mut out = [0u32; BUFSZ];
    refill_wide_impl(FakeMach, &mut state, 0, &mut out);

    assert_eq!(out, [0; BUFSZ]);
}

#[test]
#[should_panic]
fn test_refill_wide_impl_invalid_output_size() {
    struct FakeMach;

    impl Machine for FakeMach {
        type u32x4 = [u32; 4];
        type u32x4x4 = [[u32; 4]; 4];
        type u64x2 = [u64; 2];
        type u64x2x4 = [[u64; 2]; 4];

        fn vec(&self, arr: [u32; 4]) -> Self::u32x4 {
            arr
        }

        fn unpack(&self, val: vec128_storage) -> Self::u32x4 {
            val.0
        }

        fn read_le(&self, slice: &[u8]) -> Self::u32x4 {
            let arr = [
                u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]),
                u32::from_le_bytes([slice[4], slice[5], slice[6], slice[7]]),
                u32::from_le_bytes([slice[8], slice[9], slice[10], slice[11]]),
                u32::from_le_bytes([slice[12], slice[13], slice[14], slice[15]]),
            ];
            arr
        }
    }

    let mut state = ChaCha {
        b: vec128_storage([0; 4]),
        c: vec128_storage([0; 4]),
        d: vec128_storage([0; 4]),
    };
    let mut out = [0u32; 8]; // Incorrect size, should panic
    refill_wide_impl(FakeMach, &mut state, 1, &mut out);
}

#[test]
fn test_refill_wide_impl_valid_output() {
    struct FakeMach;

    impl Machine for FakeMach {
        type u32x4 = [u32; 4];
        type u32x4x4 = [[u32; 4]; 4];
        type u64x2 = [u64; 2];
        type u64x2x4 = [[u64; 2]; 4];

        fn vec(&self, arr: [u32; 4]) -> Self::u32x4 {
            arr
        }

        fn unpack(&self, val: vec128_storage) -> Self::u32x4 {
            val.0
        }

        fn read_le(&self, slice: &[u8]) -> Self::u32x4 {
            let arr = [
                u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]),
                u32::from_le_bytes([slice[4], slice[5], slice[6], slice[7]]),
                u32::from_le_bytes([slice[8], slice[9], slice[10], slice[11]]),
                u32::from_le_bytes([slice[12], slice[13], slice[14], slice[15]]),
            ];
            arr
        }
    }

    let mut state = ChaCha {
        b: vec128_storage([0; 4]),
        c: vec128_storage([0; 4]),
        d: vec128_storage([0; 4]),
    };
    let mut out = [0u32; BUFSZ];
    refill_wide_impl(FakeMach, &mut state, 1, &mut out);

    assert_ne!(out, [0; BUFSZ]);
}

