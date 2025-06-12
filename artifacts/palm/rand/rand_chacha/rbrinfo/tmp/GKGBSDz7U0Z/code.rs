pub fn get_word_pos(&self) -> u128 {
                let buf_start_block = {
                    let buf_end_block = self.rng.core.state.get_block_pos();
                    u64::wrapping_sub(buf_end_block, BUF_BLOCKS.into())
                };
                let (buf_offset_blocks, block_offset_words) = {
                    let buf_offset_words = self.rng.index() as u64;
                    let blocks_part = buf_offset_words / u64::from(BLOCK_WORDS);
                    let words_part = buf_offset_words % u64::from(BLOCK_WORDS);
                    (blocks_part, words_part)
                };
                let pos_block = u64::wrapping_add(buf_start_block, buf_offset_blocks);
                let pos_block_words = u128::from(pos_block) * u128::from(BLOCK_WORDS);
                pos_block_words + u128::from(block_offset_words)
            }