fn output_xsl_rr(state: u128) -> u64 {
    // Output function XSL RR ("xorshift low (bits), random rotation")
    // Constants are for 128-bit state, 64-bit output
    const XSHIFT: u32 = 64; // (128 - 64 + 64) / 2
    const ROTATE: u32 = 122; // 128 - 6

    let rot = (state >> ROTATE) as u32;
    let xsl = ((state >> XSHIFT) as u64) ^ (state as u64);
    xsl.rotate_right(rot)
}