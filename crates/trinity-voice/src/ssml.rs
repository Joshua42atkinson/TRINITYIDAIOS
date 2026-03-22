use trinity_protocol::VocabularyWord;

/// Generates SSML (Speech Synthesis Markup Language) text with emphasis
/// specifically targeting VAAM mastered vocabulary to create a cognitive
/// audio hook during TTS synthesis.
pub fn inject_vaam_ssml(base_text: &str, mastered_words: &[VocabularyWord]) -> String {
    let mut ssml = String::from("<speak>");

    // Simple word replacement for prototype.
    // In a production system, this would use a proper AST parser to avoid breaking HTML/XML tags.
    let mut processed_text = base_text.to_string();

    for word in mastered_words {
        let target = &word.word;
        // Apply prosody emphasis (e.g., slight volume increase and slower rate) to mastered VAAM words
        let replacement = format!(
            "<prosody volume=\"x-loud\" rate=\"slow\">{}</prosody>",
            target
        );

        // Note: this is a naive replacement and should be case-insensitive in reality.
        processed_text = processed_text.replace(target, &replacement);
    }

    ssml.push_str(&processed_text);
    ssml.push_str("</speak>");

    ssml
}
