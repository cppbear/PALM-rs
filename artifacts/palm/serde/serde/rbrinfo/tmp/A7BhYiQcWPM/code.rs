fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "{:?} or {:?}", self.tag, self.content)
        }