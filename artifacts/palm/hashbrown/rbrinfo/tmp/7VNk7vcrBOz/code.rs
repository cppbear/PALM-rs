unsafe fn drop_elements(&mut self) {
        if T::NEEDS_DROP && self.items != 0 {
            for item in self {
                item.drop();
            }
        }
    }