pub fn encode_engine<E: Engine, T: AsRef<[u8]>>(input: T, engine: &E) -> String {
    engine.encode(input)
}