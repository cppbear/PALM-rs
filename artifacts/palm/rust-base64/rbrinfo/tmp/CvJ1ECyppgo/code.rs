pub fn decode_engine_vec<E: Engine, T: AsRef<[u8]>>(
    input: T,
    buffer: &mut Vec<u8>,
    engine: &E,
) -> Result<(), DecodeError> {
    engine.decode_vec(input, buffer)
}