// Answer 0

#[test]
unsafe fn test_next_impl_valid_case() {
    let group_width = Group::WIDTH;

    let current_group = BitMaskIter(BitMask(0b0000_0000_0000_0000_0000_0000_0000_0011)); // conditions to yield indices 0, 1
    let data_bucket = Bucket { ptr: NonNull::new_unchecked(0x1000 as *mut u8) }; // mock pointer
    let next_ctrl = 0x1000 as *const u8; // initial next control
    let end = (next_ctrl as usize + group_width) as *const u8; // end within range
    
    let mut raw_iter_range = RawIterRange {
        current_group,
        data: data_bucket,
        next_ctrl,
        end,
    };

    let result: Option<Bucket<u8>> = raw_iter_range.next_impl::<false>();
}

#[test]
unsafe fn test_next_impl_check_ptr_range_true() {
    let group_width = Group::WIDTH;

    let current_group = BitMaskIter(BitMask(0b0000_0000_0000_0000_0000_0000_0000_0010)); // conditions to yield index 0
    let data_bucket = Bucket { ptr: NonNull::new_unchecked(0x1000 as *mut u8) }; // mock pointer
    let next_ctrl = (0x1000 + group_width) as *const u8; // next control aligned with group size
    let end = (0x1000 + group_width * 2) as *const u8; // valid end reference within range

    let mut raw_iter_range = RawIterRange {
        current_group,
        data: data_bucket,
        next_ctrl,
        end,
    };

    let result: Option<Bucket<u8>> = raw_iter_range.next_impl::<true>();
}

#[test]
unsafe fn test_next_impl_no_elements() {
    let group_width = Group::WIDTH;

    let current_group = BitMaskIter(BitMask(0b0000_0000_0000_0000_0000_0000_0000_0000)); // no indices available
    let data_bucket = Bucket { ptr: NonNull::new_unchecked(0x1000 as *mut u8) }; // mock pointer
    let next_ctrl = 0x1000 as *const u8;
    let end = (next_ctrl as usize + group_width) as *const u8; // end aligned within bounds

    let mut raw_iter_range = RawIterRange {
        current_group,
        data: data_bucket,
        next_ctrl,
        end,
    };

    let result: Option<Bucket<u8>> = raw_iter_range.next_impl::<false>();
}

#[test]
unsafe fn test_next_impl_next_ctrl_exceeds_end() {
    let group_width = Group::WIDTH;

    let current_group = BitMaskIter(BitMask(0b0000_0000_0000_0000_0000_0000_0000_0011)); // conditions to yield indices 0 and 1
    let data_bucket = Bucket { ptr: NonNull::new_unchecked(0x1000 as *mut u8) }; // mock pointer
    let next_ctrl = (0x2000) as *const u8; // next control exceeds end
    let end = (0x1000 as usize + group_width) as *const u8; // control end within group size

    let mut raw_iter_range = RawIterRange {
        current_group,
        data: data_bucket,
        next_ctrl,
        end,
    };

    let result: Option<Bucket<u8>> = raw_iter_range.next_impl::<true>();
}

