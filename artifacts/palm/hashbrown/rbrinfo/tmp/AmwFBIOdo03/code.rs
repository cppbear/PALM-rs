fn data_end<T>(&self) -> NonNull<T> {
        self.ctrl.cast()
    }