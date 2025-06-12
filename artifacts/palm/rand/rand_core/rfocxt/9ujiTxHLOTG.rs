use crate::RngCore;
pub fn next_u64_via_fill<R: RngCore + ?Sized>(rng: &mut R) -> u64 {
    let mut buf = [0; 8];
    rng.fill_bytes(&mut buf);
    u64::from_le_bytes(buf)
}
