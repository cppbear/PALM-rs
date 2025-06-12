fn push_wtf8_codepoint(n: u32, scratch: &mut Vec<u8>) {
    if n < 0x80 {
        scratch.push(n as u8);
        return;
    }

    scratch.reserve(4);

    // SAFETY: After the `reserve` call, `scratch` has at least 4 bytes of
    // allocated but unintialized memory after its last initialized byte, which
    // is where `ptr` points. All reachable match arms write `encoded_len` bytes
    // to that region and update the length accordingly, and `encoded_len` is
    // always <= 4.
    unsafe {
        let ptr = scratch.as_mut_ptr().add(scratch.len());

        let encoded_len = match n {
            0..=0x7F => unreachable!(),
            0x80..=0x7FF => {
                ptr.write(((n >> 6) & 0b0001_1111) as u8 | 0b1100_0000);
                2
            }
            0x800..=0xFFFF => {
                ptr.write(((n >> 12) & 0b0000_1111) as u8 | 0b1110_0000);
                ptr.add(1)
                    .write(((n >> 6) & 0b0011_1111) as u8 | 0b1000_0000);
                3
            }
            0x1_0000..=0x10_FFFF => {
                ptr.write(((n >> 18) & 0b0000_0111) as u8 | 0b1111_0000);
                ptr.add(1)
                    .write(((n >> 12) & 0b0011_1111) as u8 | 0b1000_0000);
                ptr.add(2)
                    .write(((n >> 6) & 0b0011_1111) as u8 | 0b1000_0000);
                4
            }
            0x11_0000.. => unreachable!(),
        };
        ptr.add(encoded_len - 1)
            .write((n & 0b0011_1111) as u8 | 0b1000_0000);

        scratch.set_len(scratch.len() + encoded_len);
    }
}