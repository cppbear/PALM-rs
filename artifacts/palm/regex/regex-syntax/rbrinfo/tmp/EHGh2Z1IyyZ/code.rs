fn new(trans: &'t Translator, pattern: &'p str) -> TranslatorI<'t, 'p> {
        TranslatorI { trans: trans, pattern: pattern }
    }