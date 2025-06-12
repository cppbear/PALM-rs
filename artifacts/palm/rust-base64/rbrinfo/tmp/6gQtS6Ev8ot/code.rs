pub fn encode_engine_string<E: Engine, T: AsRef<[u8]>>(
    input: T,
    output_buf: &mut String,
    engine: &E,
) {
    engine.encode_string(input, output_buf);
}