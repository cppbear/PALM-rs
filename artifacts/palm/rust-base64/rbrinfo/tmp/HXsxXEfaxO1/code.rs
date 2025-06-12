pub fn decode_engine<E: Engine, T: AsRef<[u8]>>(
    input: T,
    engine: &E,
) -> Result<Vec<u8>, DecodeError> {
    engine.decode(input)
}