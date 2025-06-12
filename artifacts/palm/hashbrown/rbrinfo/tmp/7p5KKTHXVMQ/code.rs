pub(crate) unsafe fn drop(&self) {
        self.as_ptr().drop_in_place();
    }