unsafe fn set_vec_pos(&mut self, pos: usize) {
        debug_assert_eq!(self.kind(), KIND_VEC);
        debug_assert!(pos <= MAX_VEC_POS);

        self.data = invalid_ptr((pos << VEC_POS_OFFSET) | (self.data as usize & NOT_VEC_POS_MASK));
    }