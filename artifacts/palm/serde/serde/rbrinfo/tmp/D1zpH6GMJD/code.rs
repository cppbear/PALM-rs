fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(
                formatter,
                "{:?}, {:?}, or other ignored fields",
                self.tag, self.content
            )
        }