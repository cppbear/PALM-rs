fn escape_byte(byte: u8) -> String {
    use std::ascii::escape_default;

    let escaped: Vec<u8> = escape_default(byte).collect();
    String::from_utf8_lossy(&escaped).into_owned()
}