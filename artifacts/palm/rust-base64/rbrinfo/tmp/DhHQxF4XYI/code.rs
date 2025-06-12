fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
        GeneralPurposeEstimate::new(input_len)
    }