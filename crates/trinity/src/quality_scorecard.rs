// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        quality_scorecard.rs
// PURPOSE:     Open Notebook Quality Scorecard — pedagogical document evaluation
//
// ARCHITECTURE:
//   Scores uploaded documents across 5 pedagogical dimensions:
//     1. Bloom's Coverage — are all 6 levels represented?
//     2. ADDIE Alignment — does it follow the ADDIE phases?
//     3. Accessibility — readability, structure, alt text
//     4. Student Engagement — hooks, interactivity, variety
//     5. Assessment Clarity — rubrics, measurable objectives
//
//   Scoring uses heuristic analysis (word patterns, structure detection)
//   with optional LLM-enhanced analysis for deeper evaluation.
//
//   This is Trinity's competitive differentiator:
//     NotebookLM summarizes your syllabus.
//     Trinity tells you what's missing.
//
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// Quality scorecard for an uploaded or ingested document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityScorecard {
    /// Document identifier (RAG document title or filename)
    pub document_id: String,
    /// Bloom's Coverage: 0.0-1.0 — are all 6 levels represented?
    pub blooms_coverage: f32,
    /// ADDIE Alignment: 0.0-1.0 — does the material follow ADDIE phases?
    pub addie_alignment: f32,
    /// Accessibility: 0.0-1.0 — readability, structure, heading hierarchy
    pub accessibility: f32,
    /// Student Engagement: 0.0-1.0 — hooks, interactive elements, variety
    pub engagement: f32,
    /// Assessment Clarity: 0.0-1.0 — rubrics, measurable objectives
    pub assessment_clarity: f32,
    /// Overall score: 0.0-1.0 (weighted average)
    pub overall: f32,
    /// Letter grade: A, B, C, D, F
    pub grade: String,
    /// Human-readable summary
    pub summary: String,
    /// Specific recommendations for improvement
    pub recommendations: Vec<String>,
}

impl QualityScorecard {
    /// Calculate overall score from dimensions (weighted)
    fn calculate_overall(blooms: f32, addie: f32, access: f32, engage: f32, assess: f32) -> f32 {
        // Bloom's and ADDIE are weighted higher — they're the pedagogical spine
        let weighted = blooms * 0.25 + addie * 0.25 + access * 0.15 + engage * 0.20 + assess * 0.15;
        weighted.clamp(0.0, 1.0)
    }

    /// Convert overall score to letter grade
    fn to_grade(overall: f32) -> String {
        match (overall * 100.0) as u32 {
            90..=100 => "A".to_string(),
            80..=89 => "B".to_string(),
            70..=79 => "C".to_string(),
            60..=69 => "D".to_string(),
            _ => "F".to_string(),
        }
    }

    /// Generate summary from grade
    fn to_summary(grade: &str) -> String {
        match grade {
            "A" => {
                "Excellent pedagogical alignment. Well-structured and comprehensive.".to_string()
            }
            "B" => {
                "Solid foundation with minor gaps. Ready for deployment with small improvements."
                    .to_string()
            }
            "C" => "Adequate but needs work. Key pedagogical elements are missing.".to_string(),
            "D" => "Below expectations. Significant restructuring recommended.".to_string(),
            _ => "Needs major revision. Missing core pedagogical structure.".to_string(),
        }
    }
}

// ============================================================================
// SCORING ENGINE — Heuristic Analysis
// ============================================================================

/// Bloom's taxonomy verb sets for each level
const BLOOMS_REMEMBER: &[&str] = &[
    "list",
    "define",
    "identify",
    "name",
    "recall",
    "state",
    "describe",
    "recognize",
    "label",
    "match",
];
const BLOOMS_UNDERSTAND: &[&str] = &[
    "explain",
    "summarize",
    "paraphrase",
    "classify",
    "compare",
    "interpret",
    "discuss",
    "distinguish",
];
const BLOOMS_APPLY: &[&str] = &[
    "apply",
    "demonstrate",
    "solve",
    "use",
    "implement",
    "execute",
    "calculate",
    "practice",
];
const BLOOMS_ANALYZE: &[&str] = &[
    "analyze",
    "differentiate",
    "examine",
    "categorize",
    "investigate",
    "deconstruct",
    "contrast",
    "organize",
];
const BLOOMS_EVALUATE: &[&str] = &[
    "evaluate",
    "assess",
    "judge",
    "critique",
    "justify",
    "argue",
    "defend",
    "support",
    "prioritize",
];
const BLOOMS_CREATE: &[&str] = &[
    "create",
    "design",
    "construct",
    "develop",
    "produce",
    "compose",
    "generate",
    "formulate",
    "plan",
    "invent",
];

/// ADDIE phase indicators
const ADDIE_ANALYSIS: &[&str] = &[
    "needs assessment",
    "audience",
    "learner",
    "prerequisite",
    "gap analysis",
    "survey",
    "target",
];
const ADDIE_DESIGN: &[&str] = &[
    "learning objective",
    "outcome",
    "syllabus",
    "blueprint",
    "storyboard",
    "scope",
    "sequence",
];
const ADDIE_DEVELOP: &[&str] = &[
    "content",
    "resource",
    "material",
    "activity",
    "module",
    "lesson plan",
    "worksheet",
];
const ADDIE_IMPLEMENT: &[&str] = &[
    "deliver",
    "facilitate",
    "instruct",
    "deploy",
    "launch",
    "pilot",
    "rollout",
];
const ADDIE_EVALUATE: &[&str] = &[
    "assessment",
    "rubric",
    "feedback",
    "formative",
    "summative",
    "quiz",
    "test",
    "metric",
];

/// Score a document's text for pedagogical quality.
pub fn score_document(text: &str, document_id: &str) -> QualityScorecard {
    let lower = text.to_lowercase();
    let word_count = text.split_whitespace().count();

    // 1. Bloom's Coverage
    let blooms = score_blooms_coverage(&lower);

    // 2. ADDIE Alignment
    let addie = score_addie_alignment(&lower);

    // 3. Accessibility
    let accessibility = score_accessibility(text, word_count);

    // 4. Student Engagement
    let engagement = score_engagement(&lower, word_count);

    // 5. Assessment Clarity
    let assessment = score_assessment_clarity(&lower);

    let overall =
        QualityScorecard::calculate_overall(blooms, addie, accessibility, engagement, assessment);
    let grade = QualityScorecard::to_grade(overall);
    let summary = QualityScorecard::to_summary(&grade);
    let recommendations =
        generate_recommendations(blooms, addie, accessibility, engagement, assessment);

    QualityScorecard {
        document_id: document_id.to_string(),
        blooms_coverage: blooms,
        addie_alignment: addie,
        accessibility,
        engagement,
        assessment_clarity: assessment,
        overall,
        grade,
        summary,
        recommendations,
    }
}

/// Score Bloom's coverage — check if all 6 levels have verbs present
fn score_blooms_coverage(text: &str) -> f32 {
    let levels = [
        BLOOMS_REMEMBER,
        BLOOMS_UNDERSTAND,
        BLOOMS_APPLY,
        BLOOMS_ANALYZE,
        BLOOMS_EVALUATE,
        BLOOMS_CREATE,
    ];

    let mut level_scores = Vec::new();
    for level_verbs in &levels {
        let count: usize = level_verbs
            .iter()
            .filter(|verb| text.contains(**verb))
            .count();
        // Normalize: 1+ different verbs = coverage for this level, 2+ = strong
        level_scores.push((count as f32 / 1.5).min(1.0));
    }

    // Average across all 6 levels
    level_scores.iter().sum::<f32>() / level_scores.len() as f32
}

/// Score ADDIE alignment — check all 5 phases
fn score_addie_alignment(text: &str) -> f32 {
    let phases = [
        ADDIE_ANALYSIS,
        ADDIE_DESIGN,
        ADDIE_DEVELOP,
        ADDIE_IMPLEMENT,
        ADDIE_EVALUATE,
    ];

    let mut phase_scores = Vec::new();
    for phase_terms in &phases {
        let count: usize = phase_terms
            .iter()
            .filter(|term| text.contains(**term))
            .count();
        phase_scores.push((count as f32 / 2.0).min(1.0));
    }

    phase_scores.iter().sum::<f32>() / phase_scores.len() as f32
}

/// Score accessibility — readability and structure
fn score_accessibility(text: &str, word_count: usize) -> f32 {
    let mut score = 0.5_f32; // Base

    // Sentence length heuristic (proxy for Flesch-Kincaid)
    let sentences = text.matches('.').count().max(1);
    let avg_sentence_length = word_count as f32 / sentences as f32;
    if avg_sentence_length < 20.0 {
        score += 0.15; // Good readability
    } else if avg_sentence_length > 30.0 {
        score -= 0.15; // Too complex
    }

    // Heading structure (markdown # headers)
    let heading_count = text.lines().filter(|l| l.starts_with('#')).count();
    if heading_count >= 3 {
        score += 0.2; // Good structure
    } else if heading_count >= 1 {
        score += 0.1;
    }

    // Bullet points / numbered lists
    let list_items = text
        .lines()
        .filter(|l| {
            let trimmed = l.trim();
            trimmed.starts_with('-') || trimmed.starts_with('*') || trimmed.starts_with("1.")
        })
        .count();
    if list_items >= 5 {
        score += 0.15;
    }

    score.clamp(0.0, 1.0)
}

/// Score engagement — hooks, interactivity, variety
fn score_engagement(text: &str, word_count: usize) -> f32 {
    let mut score = 0.3_f32;

    // Questions (engagement hooks)
    let question_count = text.matches('?').count();
    if question_count >= 5 {
        score += 0.2;
    } else if question_count >= 2 {
        score += 0.1;
    }

    // Activity verbs
    let activity_words = [
        "create",
        "build",
        "explore",
        "discuss",
        "investigate",
        "experiment",
        "design",
        "present",
        "collaborate",
        "brainstorm",
    ];
    let activity_count: usize = activity_words.iter().filter(|w| text.contains(**w)).count();
    score += (activity_count as f32 * 0.05).min(0.25);

    // Variety (different types of content markers)
    let has_images = text.contains("image") || text.contains("figure") || text.contains("diagram");
    let has_video = text.contains("video") || text.contains("watch") || text.contains("recording");
    let has_group = text.contains("group") || text.contains("partner") || text.contains("team");
    if has_images {
        score += 0.05;
    }
    if has_video {
        score += 0.05;
    }
    if has_group {
        score += 0.1;
    }

    // Document length bonus
    if word_count > 500 {
        score += 0.1;
    }

    score.clamp(0.0, 1.0)
}

/// Score assessment clarity
fn score_assessment_clarity(text: &str) -> f32 {
    let mut score = 0.2_f32;

    // Rubric indicators
    if text.contains("rubric") {
        score += 0.2;
    }
    if text.contains("criteria") || text.contains("criterion") {
        score += 0.1;
    }

    // Measurable language
    let measurable = [
        "percent",
        "points",
        "score",
        "grade",
        "mastery",
        "proficient",
        "benchmark",
    ];
    let measurable_count: usize = measurable.iter().filter(|w| text.contains(**w)).count();
    score += (measurable_count as f32 * 0.07).min(0.25);

    // Assessment types variety
    let types = [
        "quiz",
        "test",
        "exam",
        "portfolio",
        "project",
        "presentation",
        "essay",
    ];
    let type_count: usize = types.iter().filter(|w| text.contains(**w)).count();
    score += (type_count as f32 * 0.05).min(0.25);

    score.clamp(0.0, 1.0)
}

/// Generate actionable recommendations based on scores
fn generate_recommendations(
    blooms: f32,
    addie: f32,
    access: f32,
    engage: f32,
    assess: f32,
) -> Vec<String> {
    let mut recs = Vec::new();

    if blooms < 0.5 {
        recs.push("Low Bloom's coverage — add activities at higher cognitive levels (Analyze, Evaluate, Create).".to_string());
    }
    if addie < 0.5 {
        recs.push("ADDIE phases missing — ensure the document covers needs analysis, learning objectives, and evaluation criteria.".to_string());
    }
    if access < 0.5 {
        recs.push("Readability concerns — break long paragraphs, add headings, use bullet points for key information.".to_string());
    }
    if engage < 0.5 {
        recs.push("Low engagement — add discussion questions, collaborative activities, or multimedia references.".to_string());
    }
    if assess < 0.5 {
        recs.push("Assessment clarity is weak — add rubrics with measurable criteria and clear success benchmarks.".to_string());
    }
    if blooms > 0.7 && addie > 0.7 && engage > 0.7 {
        recs.push("Strong pedagogical foundation! Consider adding multimedia elements for further enrichment.".to_string());
    }

    if recs.is_empty() {
        recs.push("No critical issues detected. Continue refining for excellence!".to_string());
    }

    recs
}

/// L5 Sprint 6: Convert a low-scoring scorecard into quest remediation objectives.
/// When a document scores D or F, these objectives are injected into the active quest board
/// so the system *evaluates* the work and *evolves* the learner's next steps.
pub fn scorecard_to_remediation_objectives(scorecard: &QualityScorecard) -> Vec<String> {
    let mut objectives = Vec::new();

    if scorecard.blooms_coverage < 0.5 {
        objectives.push(
            "📚 Remediation: Add higher-order Bloom's activities (Analyze, Evaluate, Create) \
             to your document — current coverage is below 50%."
                .to_string(),
        );
    }
    if scorecard.addie_alignment < 0.5 {
        objectives.push(
            "🎯 Remediation: Annotate your Analysis, Design, and Evaluation sections explicitly — \
             ADDIE phase language is missing or weak."
                .to_string(),
        );
    }
    if scorecard.engagement < 0.5 {
        objectives.push(
            "🔥 Remediation: Add at least 3 discussion questions or collaborative activities — \
             engagement score is below threshold."
                .to_string(),
        );
    }
    if scorecard.assessment_clarity < 0.5 {
        objectives.push(
            "📋 Remediation: Add a rubric with measurable criteria (points, proficiency levels, benchmarks) — \
             assessment clarity needs improvement."
                .to_string(),
        );
    }
    if scorecard.accessibility < 0.5 {
        objectives.push(
            "♿ Remediation: Break long paragraphs, add heading hierarchy (#, ##), and use \
             bullet points for key information to improve readability."
                .to_string(),
        );
    }

    objectives
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_empty_document() {
        let scorecard = score_document("", "empty.txt");
        assert!(scorecard.overall < 0.5);
        assert_eq!(scorecard.grade, "F");
    }

    #[test]
    fn test_score_strong_document() {
        let text = r#"
# Ecosystems Lesson Plan

## Learning Objectives
Students will identify and describe the components of an ecosystem.
Students will analyze food web relationships and evaluate their importance.
Students will create a model ecosystem and design an experiment.

## Audience
This lesson targets 10th grade biology students with basic science prerequisite knowledge.

## Activities
- Group discussion: explore local ecosystems
- Investigation: categorize organisms by role
- Build a terrarium: construct a model ecosystem
- Present findings to the class
- Brainstorm conservation strategies

## Materials
- Textbook chapter 5
- Video: "How Ecosystems Work"
- Diagram of food webs
- Lab worksheet

## Assessment
### Rubric
| Criteria | Proficient (4) | Developing (3) | Beginning (2) |
|----------|----------------|-----------------|----------------|
| Model accuracy | 90% correct | 75% correct | 50% correct |
| Presentation | Clear, organized | Mostly clear | Disorganized |

Quiz: 20 points
Project: 50 points
Final exam: 30% of grade
"#;
        let scorecard = score_document(text, "ecosystems_lesson.md");
        assert!(
            scorecard.blooms_coverage > 0.5,
            "Bloom's should be decent: {}",
            scorecard.blooms_coverage
        );
        assert!(
            scorecard.addie_alignment > 0.5,
            "ADDIE should be covered: {}",
            scorecard.addie_alignment
        );
        assert!(
            scorecard.engagement > 0.5,
            "Engagement should be high: {}",
            scorecard.engagement
        );
        assert!(
            scorecard.assessment_clarity > 0.5,
            "Assessment should be clear: {}",
            scorecard.assessment_clarity
        );
        assert!(
            scorecard.overall > 0.5,
            "Overall should be C+ or above: {}",
            scorecard.overall
        );
        assert!(
            scorecard.grade == "A" || scorecard.grade == "B" || scorecard.grade == "C",
            "Grade: {}",
            scorecard.grade
        );
    }

    #[test]
    fn test_blooms_coverage_detects_verbs() {
        let text = "students will identify, explain, apply, analyze, evaluate, and create their own solutions";
        let score = score_blooms_coverage(text);
        assert!(score > 0.5, "All 6 levels have verbs: {}", score);
    }

    #[test]
    fn test_blooms_low_for_remember_only() {
        let text = "students will list and define and recall the vocabulary terms";
        let score = score_blooms_coverage(text);
        assert!(score < 0.5, "Only Remember level: {}", score);
    }

    #[test]
    fn test_addie_alignment() {
        let text = "needs assessment shows learner gap. learning objective is to master content. lesson plan with activities. deliver instruction through lecture. rubric for summative assessment.";
        let score = score_addie_alignment(text);
        assert!(score > 0.5, "All ADDIE phases present: {}", score);
    }

    #[test]
    fn test_recommendations_generated() {
        let scorecard = score_document("just a short note", "note.txt");
        assert!(!scorecard.recommendations.is_empty());
    }

    #[test]
    fn test_grade_boundaries() {
        assert_eq!(QualityScorecard::to_grade(0.95), "A");
        assert_eq!(QualityScorecard::to_grade(0.85), "B");
        assert_eq!(QualityScorecard::to_grade(0.75), "C");
        assert_eq!(QualityScorecard::to_grade(0.65), "D");
        assert_eq!(QualityScorecard::to_grade(0.45), "F");
    }
}
