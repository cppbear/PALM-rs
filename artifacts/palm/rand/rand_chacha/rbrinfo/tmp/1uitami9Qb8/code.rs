pub fn get_nonce(&self) -> u64 {
        get_stream_param(self, STREAM_PARAM_NONCE)
    }