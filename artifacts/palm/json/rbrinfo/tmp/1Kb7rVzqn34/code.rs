const fn build_hex_table(shift: usize) -> [i16; 256] {
    let mut table = [0; 256];
    let mut ch = 0;
    while ch < 256 {
        table[ch] = match decode_hex_val_slow(ch as u8) {
            Some(val) => (val as i16) << shift,
            None => -1,
        };
        ch += 1;
    }
    table
}