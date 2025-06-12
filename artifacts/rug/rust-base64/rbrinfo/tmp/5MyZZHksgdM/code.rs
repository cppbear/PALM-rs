fn internal_decode(
        &self,
        input: &[u8],
        output: &mut [u8],
        estimate: Self::DecodeEstimate,
    ) -> Result<DecodeMetadata, DecodeSliceError> {
        decode::decode_helper(
            input,
            &estimate,
            output,
            &self.decode_table,
            self.config.decode_allow_trailing_bits,
            self.config.decode_padding_mode,
        )
    }