fn format_escaped_str_contents<W, F>(
    writer: &mut W,
    formatter: &mut F,
    value: &str,
) -> io::Result<()>
where
    W: ?Sized + io::Write,
    F: ?Sized + Formatter,
{
    let bytes = value.as_bytes();

    let mut start = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        let escape = ESCAPE[byte as usize];
        if escape == 0 {
            continue;
        }

        if start < i {
            tri!(formatter.write_string_fragment(writer, &value[start..i]));
        }

        let char_escape = CharEscape::from_escape_table(escape, byte);
        tri!(formatter.write_char_escape(writer, char_escape));

        start = i + 1;
    }

    if start == bytes.len() {
        return Ok(());
    }

    formatter.write_string_fragment(writer, &value[start..])
}