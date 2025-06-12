pub fn fill_via_u64_chunks(src: &mut [u64], dest: &mut [u8]) -> (usize, usize) {
    fill_via_chunks(src, dest)
}