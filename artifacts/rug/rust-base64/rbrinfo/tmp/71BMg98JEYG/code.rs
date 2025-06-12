pub fn encode_engine_slice<E: Engine, T: AsRef<[u8]>>(
    input: T,
    output_buf: &mut [u8],
    engine: &E,
) -> Result<usize, EncodeSliceError> {
    engine.encode_slice(input, output_buf)
}