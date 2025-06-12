fn advance(&mut self, cnt: usize) {
        let len = self.get_ref().as_ref().len();
        let pos = self.position();

        // We intentionally allow `cnt == 0` here even if `pos > len`.
        let max_cnt = saturating_sub_usize_u64(len, pos);
        if cnt > max_cnt {
            panic_advance(&TryGetError {
                requested: cnt,
                available: max_cnt,
            });
        }

        // This will not overflow because either `cnt == 0` or the sum is not
        // greater than `len`.
        self.set_position(pos + cnt as u64);
    }