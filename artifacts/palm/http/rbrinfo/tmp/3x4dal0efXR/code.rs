fn try_reserve_one(&mut self) -> Result<(), MaxSizeReached> {
        let len = self.entries.len();

        if self.danger.is_yellow() {
            let load_factor = self.entries.len() as f32 / self.indices.len() as f32;

            if load_factor >= LOAD_FACTOR_THRESHOLD {
                // Transition back to green danger level
                self.danger.set_green();

                // Double the capacity
                let new_cap = self.indices.len() * 2;

                // Grow the capacity
                self.try_grow(new_cap)?;
            } else {
                self.danger.set_red();

                // Rebuild hash table
                for index in self.indices.iter_mut() {
                    *index = Pos::none();
                }

                self.rebuild();
            }
        } else if len == self.capacity() {
            if len == 0 {
                let new_raw_cap = 8;
                self.mask = 8 - 1;
                self.indices = vec![Pos::none(); new_raw_cap].into_boxed_slice();
                self.entries = Vec::with_capacity(usable_capacity(new_raw_cap));
            } else {
                let raw_cap = self.indices.len();
                self.try_grow(raw_cap << 1)?;
            }
        }

        Ok(())
    }