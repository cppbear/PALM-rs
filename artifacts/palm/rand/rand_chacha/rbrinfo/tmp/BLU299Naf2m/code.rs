pub fn set_word_pos(&mut self, word_offset: u128) {
                let block = (word_offset / u128::from(BLOCK_WORDS)) as u64;
                self.rng.core.state.set_block_pos(block);
                self.rng
                    .generate_and_set((word_offset % u128::from(BLOCK_WORDS)) as usize);
            }