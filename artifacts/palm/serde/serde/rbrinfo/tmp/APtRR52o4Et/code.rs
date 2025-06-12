fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, fmt::Error> {
        Err(fmt::Error)
    }