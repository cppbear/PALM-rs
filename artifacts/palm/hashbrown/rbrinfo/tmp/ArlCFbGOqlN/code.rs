unsafe fn record_item_insert_at(&mut self, index: usize, old_ctrl: Tag, hash: u64) {
        self.growth_left -= usize::from(old_ctrl.special_is_empty());
        self.set_ctrl_hash(index, hash);
        self.items += 1;
    }