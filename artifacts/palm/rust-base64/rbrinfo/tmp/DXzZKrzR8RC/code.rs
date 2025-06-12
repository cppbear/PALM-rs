pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    STANDARD.encode(input)
}