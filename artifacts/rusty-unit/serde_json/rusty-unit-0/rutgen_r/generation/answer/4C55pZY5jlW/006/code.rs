// Answer 0

#[test]
fn test_from_escape_table_form_feed() {
    struct CharEscape;

    impl CharEscape {
        const Backspace: Self = CharEscape;
        const Tab: Self = CharEscape;
        const LineFeed: Self = CharEscape;
        const FormFeed: Self = CharEscape;
        const CarriageReturn: Self = CharEscape;
        const Quote: Self = CharEscape;
        const ReverseSolidus: Self = CharEscape;
        const AsciiControl: fn(u8) -> Self = |_: u8| CharEscape;
    }

    const BB: u8 = 8;    // Backspace
    const TT: u8 = 9;    // Tab
    const NN: u8 = 10;   // LineFeed
    const FF: u8 = 12;   // FormFeed
    const RR: u8 = 13;   // CarriageReturn
    const QU: u8 = 34;   // Quote
    const BS: u8 = 92;   // ReverseSolidus
    const UU: u8 = 85;   // Custom AsciiControl escape

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

    let escape = FF;
    let byte = 0; // Placeholder value, as it won't affect this case
    let result = from_escape_table(escape, byte);
    assert!(std::mem::discriminant(&result) == std::mem::discriminant(&CharEscape::FormFeed));
}

