fn collect_str<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Display,
    {
        use self::fmt::Write;

        struct Adapter<'ser, W: 'ser, F: 'ser> {
            writer: &'ser mut W,
            formatter: &'ser mut F,
            error: Option<io::Error>,
        }

        impl<'ser, W, F> Write for Adapter<'ser, W, F>
        where
            W: io::Write,
            F: Formatter,
        {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                debug_assert!(self.error.is_none());
                match format_escaped_str_contents(self.writer, self.formatter, s) {
                    Ok(()) => Ok(()),
                    Err(err) => {
                        self.error = Some(err);
                        Err(fmt::Error)
                    }
                }
            }
        }

        tri!(self
            .formatter
            .begin_string(&mut self.writer)
            .map_err(Error::io));
        let mut adapter = Adapter {
            writer: &mut self.writer,
            formatter: &mut self.formatter,
            error: None,
        };
        match write!(adapter, "{}", value) {
            Ok(()) => debug_assert!(adapter.error.is_none()),
            Err(fmt::Error) => {
                return Err(Error::io(adapter.error.expect("there should be an error")));
            }
        }
        self.formatter
            .end_string(&mut self.writer)
            .map_err(Error::io)
    }