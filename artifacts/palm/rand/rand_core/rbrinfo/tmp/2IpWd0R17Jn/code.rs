pub fn read_u32_into(src: &[u8], dst: &mut [u32]) {
    assert!(src.len() >= 4 * dst.len());
    for (out, chunk) in dst.iter_mut().zip(src.chunks_exact(4)) {
        *out = u32::from_le_bytes(chunk.try_into().unwrap());
    }
}