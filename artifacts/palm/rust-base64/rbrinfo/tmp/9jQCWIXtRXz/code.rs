pub(crate) fn new(decoded_bytes: usize, padding_index: Option<usize>) -> Self {
        Self {
            decoded_len: decoded_bytes,
            padding_offset: padding_index,
        }
    }