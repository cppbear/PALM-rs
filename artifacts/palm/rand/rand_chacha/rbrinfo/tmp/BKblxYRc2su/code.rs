pub fn set_nonce(&mut self, value: u64) {
        set_stream_param(self, STREAM_PARAM_NONCE, value)
    }