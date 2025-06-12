fn inst_ptrs(&self) -> InstPtrs {
        InstPtrs {
            base: 0,
            data: &self.data[1..],
        }
    }