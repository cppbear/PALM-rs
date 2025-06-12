pub fn decode_engine_slice<E: Engine, T: AsRef<[u8]>>(
    input: T,
    output: &mut [u8],
    engine: &E,
) -> Result<usize, DecodeSliceError> {
    engine.decode_slice(input, output)
}