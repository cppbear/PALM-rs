unsafe fn replace_ctrl_hash(&mut self, index: usize, hash: u64) -> Tag {
        // SAFETY: The caller must uphold the safety rules for the [`RawTableInner::replace_ctrl_hash`]
        let prev_ctrl = *self.ctrl(index);
        self.set_ctrl_hash(index, hash);
        prev_ctrl
    }