fn visit_alternation_in(&mut self) -> fmt::Result {
        self.wtr.write_str("|")
    }