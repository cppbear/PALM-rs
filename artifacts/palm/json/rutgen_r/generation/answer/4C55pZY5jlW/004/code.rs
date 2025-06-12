// Answer 0

#[test]
fn test_from_escape_table_quote() {
    struct CharEscape;

    impl CharEscape {
        const Backspace: Self = CharEscape;
        const Tab: Self = CharEscape;
        const LineFeed: Self = CharEscape;
        const FormFeed: Self = CharEscape;
        const CarriageReturn: Self = CharEscape;
        const Quote: Self = CharEscape;
        const ReverseSolidus: Self = CharEscape;
        const fn AsciiControl(_byte: u8) -> Self { CharEscape }
    }

    const BB: u8 = 8; // Backspace
    const TT: u8 = 9; // Tab
    const NN: u8 = 10; // Line Feed
    const FF: u8 = 12; // Form Feed
    const RR: u8 = 13; // Carriage Return
    const QU: u8 = 34; // Quote
    const BS: u8 = 92; // Reverse Solidus
    const UU: u8 = 0; // AsciiControl

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

    let result = from_escape_table(QU, 0);
    // Testing if the result corresponds to CharEscape::Quote
    assert_eq!(std::ptr::addr_of!(result), std::ptr::addr_of!(* CharEscape::Quote)); // Pointer comparison for const
}

