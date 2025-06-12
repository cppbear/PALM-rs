pub fn set_block_pos(&mut self, value: u64) {
        set_stream_param(self, STREAM_PARAM_BLOCK, value)
    }