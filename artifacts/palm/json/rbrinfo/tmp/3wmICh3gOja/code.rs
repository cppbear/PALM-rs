fn end(self) -> Result<Value> {
        serde::ser::SerializeSeq::end(self)
    }