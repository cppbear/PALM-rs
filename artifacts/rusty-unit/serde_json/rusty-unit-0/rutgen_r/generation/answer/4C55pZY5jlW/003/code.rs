// Answer 0

#[derive(Debug, PartialEq)]
enum CharEscape {
    Backspace,
    Tab,
    LineFeed,
    FormFeed,
    CarriageReturn,
    Quote,
    ReverseSolidus,
    AsciiControl(u8),
}

const BB: u8 = 0x08; // Backspace
const TT: u8 = 0x09; // Tab
const NN: u8 = 0x0A; // Line Feed
const FF: u8 = 0x0C; // Form Feed
const RR: u8 = 0x0D; // Carriage Return
const QU: u8 = 0x22; // Double Quote
const BS: u8 = 0x5C; // Reverse Solidus
const UU: u8 = 0x75; // Unicode escape

fn from_escape_table(escape: u8, byte: u8) -> CharEscape {
    match escape {
        BB => CharEscape::Backspace,
        TT => CharEscape::Tab,
        NN => CharEscape::LineFeed,
        FF => CharEscape::FormFeed,
        RR => CharEscape::CarriageReturn,
        QU => CharEscape::Quote,
        BS => CharEscape::ReverseSolidus,
        UU => CharEscape::AsciiControl(byte),
        _ => unreachable!(),
    }
}

#[test]
fn test_reverse_solidus() {
    let escape = BS;
    let byte = 0; // byte value is not used for this variant, can be any value
    let result = from_escape_table(escape, byte);
    assert_eq!(result, CharEscape::ReverseSolidus);
}

