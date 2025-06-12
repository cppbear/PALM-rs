fn format_escaped_str<W, F>(writer: &mut W, formatter: &mut F, value: &str) -> io::Result<()>
where
    W: ?Sized + io::Write,
    F: ?Sized + Formatter,
{
    tri!(formatter.begin_string(writer));
    tri!(format_escaped_str_contents(writer, formatter, value));
    formatter.end_string(writer)
}