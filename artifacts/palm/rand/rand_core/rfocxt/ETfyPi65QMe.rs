#[inline]
#[track_caller]
pub fn read_u64_into(src: &[u8], dst: &mut [u64]) {
    assert!(src.len() >= 8 * dst.len());
    for (out, chunk) in dst.iter_mut().zip(src.chunks_exact(8)) {
        *out = u64::from_le_bytes(chunk.try_into().unwrap());
    }
}
