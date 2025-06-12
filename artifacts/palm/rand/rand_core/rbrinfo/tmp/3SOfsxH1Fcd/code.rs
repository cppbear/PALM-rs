pub fn next_u32_via_fill<R: RngCore + ?Sized>(rng: &mut R) -> u32 {
    let mut buf = [0; 4];
    rng.fill_bytes(&mut buf);
    u32::from_le_bytes(buf)
}