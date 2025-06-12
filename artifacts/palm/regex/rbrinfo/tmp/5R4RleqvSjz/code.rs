fn set_word_boundary(&mut self) {
        // We need to mark all ranges of bytes whose pairs result in
        // evaluating \b differently.
        let iswb = is_word_byte;
        let mut b1: u16 = 0;
        let mut b2: u16;
        while b1 <= 255 {
            b2 = b1 + 1;
            while b2 <= 255 && iswb(b1 as u8) == iswb(b2 as u8) {
                b2 += 1;
            }
            self.set_range(b1 as u8, (b2 - 1) as u8);
            b1 = b2;
        }
    }