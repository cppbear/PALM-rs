fn read_u64(s: &[u8]) -> u64 {
    u64::from_be_bytes(s[..8].try_into().unwrap())
}