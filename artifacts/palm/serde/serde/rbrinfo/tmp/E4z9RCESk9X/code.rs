fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let mut buf = [0u8; 58];
        let mut writer = crate::format::Buf::new(&mut buf);
        fmt::Write::write_fmt(&mut writer, format_args!("integer `{}` as i128", v)).unwrap();
        Err(Error::invalid_type(
            Unexpected::Other(writer.as_str()),
            &self,
        ))
    }