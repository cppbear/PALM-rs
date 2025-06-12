fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool {
        use prog::EmptyLook::*;
        match empty.look {
            StartLine => {
                let c = self.previous_char(at);
                at.pos() == 0 || c == '\n'
            }
            EndLine => {
                let c = self.next_char(at);
                at.pos() == self.len() || c == '\n'
            }
            StartText => at.pos() == 0,
            EndText => at.pos() == self.len(),
            WordBoundary => {
                let (c1, c2) = (self.previous_char(at), self.next_char(at));
                c1.is_word_char() != c2.is_word_char()
            }
            NotWordBoundary => {
                let (c1, c2) = (self.previous_char(at), self.next_char(at));
                c1.is_word_char() == c2.is_word_char()
            }
            WordBoundaryAscii => {
                let (c1, c2) = (self.previous_char(at), self.next_char(at));
                if self.only_utf8 {
                    // If we must match UTF-8, then we can't match word
                    // boundaries at invalid UTF-8.
                    if c1.is_none() && !at.is_start() {
                        return false;
                    }
                    if c2.is_none() && !at.is_end() {
                        return false;
                    }
                }
                c1.is_word_byte() != c2.is_word_byte()
            }
            NotWordBoundaryAscii => {
                let (c1, c2) = (self.previous_char(at), self.next_char(at));
                if self.only_utf8 {
                    // If we must match UTF-8, then we can't match word
                    // boundaries at invalid UTF-8.
                    if c1.is_none() && !at.is_start() {
                        return false;
                    }
                    if c2.is_none() && !at.is_end() {
                        return false;
                    }
                }
                c1.is_word_byte() == c2.is_word_byte()
            }
        }
    }