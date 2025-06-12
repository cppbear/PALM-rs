fn remaining(&self) -> usize {
        saturating_sub_usize_u64(self.get_ref().as_ref().len(), self.position())
    }