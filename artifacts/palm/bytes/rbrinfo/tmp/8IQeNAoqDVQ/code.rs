fn offset_from(dst: *const u8, original: *const u8) -> usize {
    dst as usize - original as usize
}