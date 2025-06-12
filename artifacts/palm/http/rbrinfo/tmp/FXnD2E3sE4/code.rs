fn desired_pos(mask: Size, hash: HashValue) -> usize {
    (hash.0 & mask) as usize
}