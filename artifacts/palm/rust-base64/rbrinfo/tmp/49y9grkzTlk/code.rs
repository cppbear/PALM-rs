pub fn decoded_len_estimate(encoded_len: usize) -> usize {
    STANDARD
        .internal_decoded_len_estimate(encoded_len)
        .decoded_len_estimate()
}