fn at(&self, i: usize) -> InputAt {
        let c = decode_utf8(&self[i..]).map(|(c, _)| c).into();
        InputAt {
            pos: i,
            c: c,
            byte: None,
            len: c.len_utf8(),
        }
    }