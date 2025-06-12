fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct WriterFormatter<'a, 'b: 'a> {
            inner: &'a mut fmt::Formatter<'b>,
        }

        impl<'a, 'b> io::Write for WriterFormatter<'a, 'b> {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                // Safety: the serializer below only emits valid utf8 when using
                // the default formatter.
                let s = unsafe { str::from_utf8_unchecked(buf) };
                tri!(self.inner.write_str(s).map_err(io_error));
                Ok(buf.len())
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        fn io_error(_: fmt::Error) -> io::Error {
            // Error value does not matter because Display impl just maps it
            // back to fmt::Error.
            io::Error::new(io::ErrorKind::Other, "fmt error")
        }

        let alternate = f.alternate();
        let mut wr = WriterFormatter { inner: f };
        if alternate {
            // {:#}
            super::ser::to_writer_pretty(&mut wr, self).map_err(|_| fmt::Error)
        } else {
            // {}
            super::ser::to_writer(&mut wr, self).map_err(|_| fmt::Error)
        }
    }