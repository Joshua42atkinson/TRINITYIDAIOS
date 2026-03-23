import React, { useState } from 'react';

/**
 * Quality Scorecard — pedagogical document evaluation UI
 *
 * Calls POST /api/yard/score with either inline text or a document_id,
 * displays the 5-dimension scorecard with visual bars and recommendations.
 */

const SAMPLE_TEXT = `Lesson Plan: Introduction to Photosynthesis (7th Grade Biology)

Objectives:
- Students will identify the reactants and products of photosynthesis (Remember)
- Students will explain the role of chlorophyll in light absorption (Understand)
- Students will compare photosynthesis and cellular respiration (Analyze)
- Students will design an experiment testing light intensity's effect on plant growth (Create)

Materials: Elodea plants, beakers, sodium bicarbonate solution, light source, ruler, data sheets

Instructional Sequence:
1. Hook (5 min): Show time-lapse video of plant growth. Ask: "Where does the mass of a tree come from?"
2. Direct Instruction (15 min): Diagram the light-dependent and light-independent reactions. Key vocabulary: chloroplast, thylakoid, stroma, ATP, NADPH, Calvin cycle.
3. Guided Practice (10 min): Students label a photosynthesis diagram and trace the path of a carbon atom from CO2 to glucose.
4. Collaborative Activity (15 min): In pairs, students design a controlled experiment varying light intensity. They must write a hypothesis, identify variables, and predict outcomes.
5. Closure (5 min): Exit ticket — "Explain in 2 sentences why photosynthesis matters for life on Earth."

Assessment:
- Formative: Exit ticket, diagram labeling accuracy
- Summative: Lab report with rubric (hypothesis clarity, data collection, analysis, conclusion)
- Rubric criteria: Scientific accuracy (25%), Experimental design (25%), Data analysis (25%), Written communication (25%)

Differentiation:
- ELL students receive bilingual vocabulary cards
- Advanced learners explore C4 and CAM photosynthesis pathways
- Students with IEPs receive graphic organizers with partially completed diagrams

Standards Alignment: NGSS MS-LS1-6, Bloom's Taxonomy levels 1-6 addressed`;

const DIMENSIONS = [
  { key: 'blooms_coverage', label: "Bloom's Coverage", icon: '🧠', color: '#cfb991', desc: 'Are all 6 cognitive levels represented?' },
  { key: 'addie_alignment', label: 'ADDIE Alignment', icon: '📐', color: '#569cd6', desc: 'Does the document follow ADDIE phases?' },
  { key: 'accessibility', label: 'Accessibility', icon: '♿', color: '#4ec9b0', desc: 'Readability, structure, heading hierarchy' },
  { key: 'engagement', label: 'Engagement', icon: '🎯', color: '#c586c0', desc: 'Hooks, interactivity, variety' },
  { key: 'assessment_clarity', label: 'Assessment Clarity', icon: '📊', color: '#dcdcaa', desc: 'Rubrics, measurable objectives' },
];

function gradeColor(grade) {
  switch (grade) {
    case 'A': return '#4ec9b0';
    case 'B': return '#569cd6';
    case 'C': return '#cfb991';
    case 'D': return '#ce9178';
    default: return '#f44747';
  }
}

function ScoreBar({ score, color }) {
  const pct = Math.round(score * 100);
  return (
    <div className="scorecard-bar">
      <div
        className="scorecard-bar__fill"
        style={{
          width: `${pct}%`,
          background: `linear-gradient(to right, ${color}88, ${color})`,
        }}
      />
      <span className="scorecard-bar__label">{pct}%</span>
    </div>
  );
}

export default function QualityScorecard() {
  const [text, setText] = useState('');
  const [docId, setDocId] = useState('');
  const [scorecard, setScorecard] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [inputMode, setInputMode] = useState('paste'); // 'paste' or 'rag'

  const handleScore = async () => {
    setLoading(true);
    setError(null);
    setScorecard(null);

    const body = inputMode === 'paste'
      ? { text, document_id: '' }
      : { text: '', document_id: docId };

    try {
      const res = await fetch('/api/yard/score', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body),
      });
      if (!res.ok) {
        const errText = await res.text();
        throw new Error(errText || `HTTP ${res.status}`);
      }
      const data = await res.json();
      setScorecard(data);
    } catch (err) {
      setError(err.message);
    }
    setLoading(false);
  };

  const handleTryExample = () => {
    setInputMode('paste');
    setText(SAMPLE_TEXT);
  };

  return (
    <div className="scorecard-panel">
      <div className="scorecard-header">
        <div className="scorecard-header__icon">📋</div>
        <div>
          <div className="scorecard-header__title">QUALITY SCORECARD</div>
          <div className="scorecard-header__subtitle">
            Pedagogical document evaluation — 5 dimensions of instructional quality
          </div>
        </div>
      </div>

      {/* Input Area */}
      <div className="scorecard-input-section">
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <div className="scorecard-mode-toggle">
            <button
              className={`scorecard-mode-btn ${inputMode === 'paste' ? 'scorecard-mode-btn--active' : ''}`}
              onClick={() => setInputMode('paste')}
            >
              📝 Paste Text
            </button>
            <button
              className={`scorecard-mode-btn ${inputMode === 'rag' ? 'scorecard-mode-btn--active' : ''}`}
              onClick={() => setInputMode('rag')}
            >
              🔍 RAG Lookup
            </button>
          </div>
          {!text && inputMode === 'paste' && (
            <button
              className="scorecard-mode-btn scorecard-mode-btn--active"
              onClick={handleTryExample}
              style={{ fontSize: '0.75rem' }}
            >
              🧪 Try an Example
            </button>
          )}
        </div>

        {inputMode === 'paste' ? (
          <textarea
            className="scorecard-textarea"
            placeholder="Paste your lesson plan, syllabus, or educational document here..."
            value={text}
            onChange={(e) => setText(e.target.value)}
            rows={6}
          />
        ) : (
          <input
            className="scorecard-doc-input"
            placeholder="Enter document title to look up in RAG..."
            value={docId}
            onChange={(e) => setDocId(e.target.value)}
          />
        )}

        <button
          className="scorecard-submit"
          disabled={loading || (inputMode === 'paste' ? !text.trim() : !docId.trim())}
          onClick={handleScore}
        >
          {loading ? '⏳ Scoring...' : '📊 Score Document'}
        </button>

        {error && (
          <div className="scorecard-error">⚠️ {error}</div>
        )}
      </div>

      {/* Results */}
      {scorecard && (
        <div className="scorecard-results">
          {/* Grade Circle */}
          <div className="scorecard-grade-section">
            <div
              className="scorecard-grade-circle"
              style={{
                borderColor: gradeColor(scorecard.grade),
                boxShadow: `0 0 30px ${gradeColor(scorecard.grade)}33`,
              }}
            >
              <div className="scorecard-grade-letter" style={{ color: gradeColor(scorecard.grade) }}>
                {scorecard.grade}
              </div>
              <div className="scorecard-grade-pct">
                {Math.round(scorecard.overall * 100)}%
              </div>
            </div>
            <div className="scorecard-summary">{scorecard.summary}</div>
            <div className="scorecard-doc-id">
              📄 {scorecard.document_id}
            </div>
          </div>

          {/* Dimension Bars */}
          <div className="scorecard-dimensions">
            {DIMENSIONS.map((dim) => (
              <div key={dim.key} className="scorecard-dimension">
                <div className="scorecard-dimension__header">
                  <span className="scorecard-dimension__icon">{dim.icon}</span>
                  <span className="scorecard-dimension__label">{dim.label}</span>
                </div>
                <ScoreBar score={scorecard[dim.key]} color={dim.color} />
                <div className="scorecard-dimension__desc">{dim.desc}</div>
              </div>
            ))}
          </div>

          {/* Recommendations */}
          {scorecard.recommendations?.length > 0 && (
            <div className="scorecard-recs">
              <div className="scorecard-recs__header">💡 RECOMMENDATIONS</div>
              {scorecard.recommendations.map((rec, i) => (
                <div key={i} className="scorecard-rec-item">
                  <span className="scorecard-rec-item__bullet">▸</span>
                  {rec}
                </div>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
