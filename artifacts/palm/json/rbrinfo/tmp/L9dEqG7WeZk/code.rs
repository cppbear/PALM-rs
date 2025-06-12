fn from(f: Cow<'a, str>) -> Self {
        Value::String(f.into_owned())
    }