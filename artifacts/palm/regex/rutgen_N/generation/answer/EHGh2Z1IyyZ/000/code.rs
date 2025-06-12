// Answer 0

#[test]
fn test_new_translator_i() {
    struct Translator;

    struct TranslatorI<'t, 'p> {
        trans: &'t Translator,
        pattern: &'p str,
    }

    let translator = Translator;
    let pattern = "example_pattern";

    let translator_i = new(&translator, pattern);

    assert_eq!(translator_i.pattern, pattern);
    assert_eq!(translator_i.trans, &translator);
}

