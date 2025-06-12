pub fn get_block_pos(&self) -> u64 {
        get_stream_param(self, STREAM_PARAM_BLOCK)
    }