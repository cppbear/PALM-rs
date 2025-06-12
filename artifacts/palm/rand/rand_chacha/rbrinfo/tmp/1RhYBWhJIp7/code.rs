pub fn get_seed(&self) -> [u8; 32] {
                self.rng.core.state.get_seed()
            }