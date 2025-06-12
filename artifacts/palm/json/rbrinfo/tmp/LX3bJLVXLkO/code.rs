fn from(j: Error) -> Self {
        if let ErrorCode::Io(err) = j.err.code {
            err
        } else {
            match j.classify() {
                Category::Io => unreachable!(),
                Category::Syntax | Category::Data => io::Error::new(ErrorKind::InvalidData, j),
                Category::Eof => io::Error::new(ErrorKind::UnexpectedEof, j),
            }
        }
    }