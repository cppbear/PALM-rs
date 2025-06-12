unsafe fn get_vec_pos(&self) -> usize {
        debug_assert_eq!(self.kind(), KIND_VEC);

        self.data as usize >> VEC_POS_OFFSET
    }