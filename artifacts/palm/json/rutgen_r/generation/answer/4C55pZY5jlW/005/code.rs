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

const BB: u8 = 8; 
const TT: u8 = 9; 
const NN: u8 = 10; 
const FF: u8 = 12; 
const RR: u8 = 13; 
const QU: u8 = 34; 
const BS: u8 = 92; 
const UU: u8 = 0; 

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
fn test_from_escape_table_carriage_return() {
    let escape_value = RR;
    let byte_value = 0; // Arbitrary, will not be used since RR matches
    let expected = CharEscape::CarriageReturn;
    let result = from_escape_table(escape_value, byte_value);
    assert_eq!(result, expected);
}

