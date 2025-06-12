// Answer 0

#[test]
fn test_to_le_bytes_for_u32() {
    // Auxiliary struct to mimic the functionality of the trait.
    struct TestU32(u32);

    impl TestU32 {
        fn to_le_bytes(self) -> [u8; 4] {
            self.0.to_le_bytes()
        }
    }

    let input = TestU32(0x12345678);
    let expected_output = [0x78, 0x56, 0x34, 0x12];
    assert_eq!(input.to_le_bytes(), expected_output);
}

#[test]
fn test_to_le_bytes_for_u64() {
    // Auxiliary struct to mimic the functionality of the trait.
    struct TestU64(u64);

    impl TestU64 {
        fn to_le_bytes(self) -> [u8; 8] {
            self.0.to_le_bytes()
        }
    }

    let input = TestU64(0x1234567890abcdef);
    let expected_output = [0xef, 0xcd, 0xab, 0x90, 0x78, 0x56, 0x34, 0x12];
    assert_eq!(input.to_le_bytes(), expected_output);
}

#[test]
fn test_to_le_bytes_for_u16() {
    // Auxiliary struct to mimic the functionality of the trait.
    struct TestU16(u16);

    impl TestU16 {
        fn to_le_bytes(self) -> [u8; 2] {
            self.0.to_le_bytes()
        }
    }

    let input = TestU16(0x1234);
    let expected_output = [0x34, 0x12];
    assert_eq!(input.to_le_bytes(), expected_output);
}

#[test]
fn test_to_le_bytes_for_u8() {
    // Auxiliary struct to mimic the functionality of the trait.
    struct TestU8(u8);

    impl TestU8 {
        fn to_le_bytes(self) -> [u8; 1] {
            [self.0]
        }
    }

    let input = TestU8(0x12);
    let expected_output = [0x12];
    assert_eq!(input.to_le_bytes(), expected_output);
}

