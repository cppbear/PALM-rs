unsafe fn set_ctrl_hash(&mut self, index: usize, hash: u64) {
        // SAFETY: The caller must uphold the safety rules for the [`RawTableInner::set_ctrl_hash`]
        self.set_ctrl(index, Tag::full(hash));
    }