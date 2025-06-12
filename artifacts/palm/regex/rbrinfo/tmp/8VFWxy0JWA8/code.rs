fn from(x: &'a str) -> Ref<'a> {
        Ref::Named(x)
    }