pub fn get_stream(&self) -> u64 {
                self.rng.core.state.get_nonce()
            }