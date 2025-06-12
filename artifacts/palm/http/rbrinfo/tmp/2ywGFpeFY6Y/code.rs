pub fn as_str(&self) -> &str {
        let ret = &self.data[..];
        if ret.is_empty() {
            return "/";
        }
        ret
    }