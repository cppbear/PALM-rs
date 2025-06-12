fn fix_position(&self, err: Error) -> Error {
        err.fix_position(move |code| self.error(code))
    }