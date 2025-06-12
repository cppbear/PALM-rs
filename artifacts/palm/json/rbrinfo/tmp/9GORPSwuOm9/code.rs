fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        tri!(writer.write_all(if first { b"\n" } else { b",\n" }));
        indent(writer, self.current_indent, self.indent)
    }