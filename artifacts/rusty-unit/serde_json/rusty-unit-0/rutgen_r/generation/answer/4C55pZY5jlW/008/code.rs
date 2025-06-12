// Answer 0

#[test]
fn test_from_escape_table_tab() {
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

    const BB: u8 = 0x08;
    const TT: u8 = 0x09;
    const NN: u8 = 0x0A;
    const FF: u8 = 0x0C;
    const RR: u8 = 0x0D;
    const QU: u8 = 0x22;
    const BS: u8 = 0x5C;
    const UU: u8 = 0x00;

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

    let escape = TT;
    let byte = 0; // byte is not used when escape is TT
    let result = from_escape_table(escape, byte);
    assert_eq!(result as *const _ as usize, CharEscape::Tab as *const _ as usize);
}

