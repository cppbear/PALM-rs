pub(crate) const fn encode_table(alphabet: &Alphabet) -> [u8; 64] {
    // the encode table is just the alphabet:
    // 6-bit index lookup -> printable byte
    let mut encode_table = [0_u8; 64];
    {
        let mut index = 0;
        while index < 64 {
            encode_table[index] = alphabet.symbols[index];
            index += 1;
        }
    }

    encode_table
}