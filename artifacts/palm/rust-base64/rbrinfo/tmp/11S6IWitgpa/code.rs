pub(crate) fn add_padding(unpadded_output_len: usize, output: &mut [u8]) -> usize {
    let pad_bytes = (4 - (unpadded_output_len % 4)) % 4;
    // for just a couple bytes, this has better performance than using
    // .fill(), or iterating over mutable refs, which call memset()
    #[allow(clippy::needless_range_loop)]
    for i in 0..pad_bytes {
        output[i] = PAD_BYTE;
    }

    pad_bytes
}