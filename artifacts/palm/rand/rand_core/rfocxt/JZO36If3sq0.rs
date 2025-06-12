use crate::RngCore;
#[deprecated(since = "0.9.3", note = "use BlockRng instead")]
pub fn fill_via_u32_chunks(src: &mut [u32], dest: &mut [u8]) -> (usize, usize) {
    fill_via_chunks(src, dest)
}
pub(crate) fn fill_via_chunks<T: Observable>(
    src: &[T],
    dest: &mut [u8],
) -> (usize, usize) {
    let size = core::mem::size_of::<T>();
    let mut dest = dest.chunks_exact_mut(size);
    let mut src = src.iter();
    let zipped = dest.by_ref().zip(src.by_ref());
    let num_chunks = zipped.len();
    zipped.for_each(|(dest, src)| dest.copy_from_slice(src.to_le_bytes().as_ref()));
    let byte_len = num_chunks * size;
    if let Some(src) = src.next() {
        let dest = dest.into_remainder();
        let n = dest.len();
        if n > 0 {
            dest.copy_from_slice(&src.to_le_bytes().as_ref()[..n]);
            return (num_chunks + 1, byte_len + n);
        }
    }
    (num_chunks, byte_len)
}
