// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Voice Pipeline
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:         ssml.rs
// BIBLE CAR:    Car 11 — YOKE (ART Pipeline & Creative Tools, §11.4)
// HOOK SCHOOL:  🎨 Creation — Voice Pipeline
// PURPOSE:      SSML (Speech Synthesis Markup Language) injection for VAAM-aware
//               TTS. When Pete speaks, mastered vocabulary words receive prosody
//               emphasis (louder, slower) — creating an audio cognitive hook that
//               reinforces learning through auditory repetition. This is the
//               audio equivalent of bolding a word in text.
//
// ARCHITECTURE:
//   • inject_vaam_ssml() wraps mastered words in <prosody> tags
//   • Output feeds into Supertonic-2 TTS or Piper TTS synthesis
//   • Naive word replacement — production needs case-insensitive AST parser
//   • Bible Car 11.4: "Walkie-Talkie" pipeline (Whisper STT → Pete → TTS)
//
// DEPENDENCIES:
//   - trinity_protocol — VocabularyWord type
//
// CHANGES:
//   2026-03-16  Joshua Atkinson  Created for VAAM-aware TTS
//   2026-03-26  Cascade          Added §17 header
//
// ═══════════════════════════════════════════════════════════════════════════════

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
