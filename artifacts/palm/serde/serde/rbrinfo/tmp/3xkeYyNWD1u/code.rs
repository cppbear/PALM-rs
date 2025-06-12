fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            write!(fmt, "a type tag `{}` or any other value", self.name)
        }