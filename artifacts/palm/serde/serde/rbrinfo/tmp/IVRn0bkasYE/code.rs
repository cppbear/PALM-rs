fn format_u8(mut n: u8, out: &mut [u8]) -> usize {
    if n >= 100 {
        let d1 = ((n % 100) << 1) as usize;
        n /= 100;
        out[0] = b'0' + n;
        out[1] = DEC_DIGITS_LUT[d1];
        out[2] = DEC_DIGITS_LUT[d1 + 1];
        3
    } else if n >= 10 {
        let d1 = (n << 1) as usize;
        out[0] = DEC_DIGITS_LUT[d1];
        out[1] = DEC_DIGITS_LUT[d1 + 1];
        2
    } else {
        out[0] = b'0' + n;
        1
    }
}