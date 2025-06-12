fn from(opt: Option<T>) -> Self {
        match opt {
            None => Value::Null,
            Some(value) => Into::into(value),
        }
    }