pub fn contains_control_chars(string: &str) -> bool {
    string.chars().any(|c| c.is_control())
}

static NORMALIZER_NFC: icu_normalizer::ComposingNormalizerBorrowed =
    icu_normalizer::ComposingNormalizerBorrowed::new_nfc();

pub fn normalize_nfc(string: &str) -> String {
    NORMALIZER_NFC.normalize(string).into()
}
