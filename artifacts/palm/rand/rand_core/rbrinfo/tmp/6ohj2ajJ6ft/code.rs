pub fn fill_via_u32_chunks(src: &mut [u32], dest: &mut [u8]) -> (usize, usize) {
    fill_via_chunks(src, dest)
}