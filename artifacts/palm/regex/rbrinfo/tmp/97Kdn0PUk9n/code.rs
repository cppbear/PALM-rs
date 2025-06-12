fn next_after_empty(&self, text: &str, i: usize) -> usize {
        next_utf8(text.as_bytes(), i)
    }