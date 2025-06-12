// Answer 0

#[test]
fn test_invert() {
    struct BitMask(u64);

    const BITMASK_MASK: u64 = 0xFFFFFFFFFFFFFFFF;

    impl BitMask {
        pub(crate) fn invert(self) -> Self {
            BitMask(self.0 ^ BITMASK_MASK)
        }
    }

    // Test case: Inverting a BitMask with all bits set to 0
    let mask_zero = BitMask(0);
    let inverted_zero = mask_zero.invert();
    assert_eq!(inverted_zero.0, BITMASK_MASK);

    // Test case: Inverting a BitMask with all bits set to 1
    let mask_full = BitMask(BITMASK_MASK);
    let inverted_full = mask_full.invert();
    assert_eq!(inverted_full.0, 0);

    // Test case: Inverting a BitMask with some bits set
    let mask_some = BitMask(0b10101010);
    let inverted_some = mask_some.invert();
    assert_eq!(inverted_some.0, BITMASK_MASK ^ 0b10101010);
}

