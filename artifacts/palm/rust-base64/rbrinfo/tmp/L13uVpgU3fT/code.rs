pub(crate) const fn decode_table(alphabet: &Alphabet) -> [u8; 256] {
    let mut decode_table = [INVALID_VALUE; 256];

    // Since the table is full of `INVALID_VALUE` already, we only need to overwrite
    // the parts that are valid.
    let mut index = 0;
    while index < 64 {
        // The index in the alphabet is the 6-bit value we care about.
        // Since the index is in 0-63, it is safe to cast to u8.
        decode_table[alphabet.symbols[index] as usize] = index as u8;
        index += 1;
    }

    decode_table
}