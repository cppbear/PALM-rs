fn start_flags(&self, text: &[u8], at: usize) -> (EmptyFlags, StateFlags) {
        let mut empty_flags = EmptyFlags::default();
        let mut state_flags = StateFlags::default();
        empty_flags.start = at == 0;
        empty_flags.end = text.is_empty();
        empty_flags.start_line = at == 0 || text[at - 1] == b'\n';
        empty_flags.end_line = text.is_empty();

        let is_word_last = at > 0 && Byte::byte(text[at - 1]).is_ascii_word();
        let is_word = at < text.len() && Byte::byte(text[at]).is_ascii_word();
        if is_word_last {
            state_flags.set_word();
        }
        if is_word == is_word_last {
            empty_flags.not_word_boundary = true;
        } else {
            empty_flags.word_boundary = true;
        }
        (empty_flags, state_flags)
    }