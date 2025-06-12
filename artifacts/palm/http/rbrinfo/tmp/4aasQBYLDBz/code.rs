fn probe_distance(mask: Size, hash: HashValue, current: usize) -> usize {
    current.wrapping_sub(desired_pos(mask, hash)) & mask as usize
}