// Answer 0

#[test]
fn test_from_escape_table_tab() {
    struct CharEscape;

    impl CharEscape {
        const Backspace: CharEscape = CharEscape;
        const Tab: CharEscape = CharEscape;
        const LineFeed: CharEscape = CharEscape;
        const FormFeed: CharEscape = CharEscape;
        const CarriageReturn: CharEscape = CharEscape;
        const Quote: CharEscape = CharEscape;
        const ReverseSolidus: CharEscape = CharEscape;
        const AsciiControl: fn(u8) -> CharEscape = |byte| CharEscape;
    }

    const BB: u8 = 8;  // Backspace
    const TT: u8 = 9;  // Tab
    const NN: u8 = 10; // Line Feed
    const FF: u8 = 12; // Form Feed
    const RR: u8 = 13; // Carriage Return
    const QU: u8 = 34; // Quote
    const BS: u8 = 92; // Reverse Solidus
    const UU: u8 = 0;  // AsciiControl

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

    let result = from_escape_table(TT, 0);
    assert_eq!(result, CharEscape::Tab);
}

