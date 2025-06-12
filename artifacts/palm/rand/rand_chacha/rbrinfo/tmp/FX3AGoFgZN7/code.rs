pub fn set_stream(&mut self, stream: u64) {
                self.rng.core.state.set_nonce(stream);
                if self.rng.index() != 64 {
                    let wp = self.get_word_pos();
                    self.set_word_pos(wp);
                }
            }