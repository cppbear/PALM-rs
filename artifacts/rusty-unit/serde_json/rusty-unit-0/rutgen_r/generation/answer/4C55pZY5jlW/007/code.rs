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

const BB: u8 = 0x08;
const TT: u8 = 0x09;
const NN: u8 = 0x0A;
const FF: u8 = 0x0C;
const RR: u8 = 0x0D;
const QU: u8 = 0x22;
const BS: u8 = 0x5C;
const UU: u8 = 0x75;

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
fn test_from_escape_table_line_feed() {
    let escape = NN;
    let byte = 0; // Byte is irrelevant for this test as we expect LineFeed
    let result = from_escape_table(escape, byte);
    assert_eq!(result, CharEscape::LineFeed);
}

