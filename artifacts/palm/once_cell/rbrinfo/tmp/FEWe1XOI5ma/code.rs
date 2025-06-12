fn from_usize(value: NonZeroUsize) -> bool {
        value.get() == 1
    }