fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }