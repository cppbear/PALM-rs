fn from(x: usize) -> Ref<'static> {
        Ref::Number(x)
    }