pub(crate) fn new(encoded_len: usize) -> Self {
        let rem = encoded_len % 4;
        Self {
            rem,
            conservative_decoded_len: (encoded_len / 4 + usize::from(rem > 0)) * 3,
        }
    }