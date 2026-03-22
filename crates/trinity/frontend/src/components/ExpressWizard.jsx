import React, { useState } from 'react';

const SUBJECTS = [
  'Ecosystems', 'Physics', 'US History', 'Algebra',
  'Health Science', 'Creative Writing', 'Chemistry', 'Computer Science',
  'Art History', 'Music', 'Physical Education', 'Social Studies',
];

const GRADES = ['K-2', '3-5', '6-8', '9-12', 'College', 'Professional'];

const FORMATS = [
  { value: 'quiz', label: '📝 Quiz', desc: 'Multiple choice, true/false, short answer' },
  { value: 'adventure', label: '🗺️ Adventure', desc: 'Choose-your-own-path story game' },
  { value: 'flashcards', label: '🃏 Flashcards', desc: 'Vocabulary drill with spaced repetition' },
  { value: 'lesson_plan', label: '📋 Lesson Plan', desc: 'Bloom\'s-aligned lesson with activities' },
];

export default function ExpressWizard({ onComplete }) {
  const [step, setStep] = useState(1);
  const [data, setData] = useState({
    subject: '', customSubject: '', grade: '9-12',
    goal: '', format: 'quiz', audience: '',
  });
  const [generating, setGenerating] = useState(false);
  const [result, setResult] = useState(null);

  const subject = data.subject || data.customSubject;

  const handleGenerate = async () => {
    if (!subject || !data.goal) return;
    setGenerating(true);

    try {
      // Create PEARL
      await fetch('/api/pearl', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          subject,
          medium: data.format,
          vision: data.goal,
        }),
      });

      // Update character sheet
      await fetch('/api/character', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          experience: 'Express mode user',
          audience: data.audience || `${data.grade} students`,
          success_vision: data.goal,
        }),
      });

      // Compile GDD from express data
      const gddRes = await fetch('/api/quest/compile', { method: 'POST' });
      const gddData = await gddRes.json().catch(() => ({}));

      setResult({
        title: `${subject} — ${data.format}`,
        format: data.format,
        gdd: gddData,
        subject,
        grade: data.grade,
        goal: data.goal,
      });
    } catch (err) {
      console.error('Express generation failed:', err);
    }
    setGenerating(false);
  };

  // ── Step 1: What are you teaching? ──
  if (step === 1) {
    return (
      <div className="express-wizard">
        <div className="express-wizard__header">
          <div className="express-wizard__icon">⚡</div>
          <div className="express-wizard__title">EXPRESS MODE</div>
          <div className="express-wizard__subtitle">
            Answer three questions. Get a game in ten minutes.
          </div>
        </div>

        <div className="express-wizard__step">
          <div className="express-wizard__step-label">STEP 1 OF 3</div>
          <div className="express-wizard__question">What are you teaching?</div>

          <div className="subject-grid">
            {SUBJECTS.map((s) => (
              <button
                key={s}
                className={`subject-btn ${data.subject === s ? 'subject-btn--active' : ''}`}
                onClick={() => setData({ ...data, subject: s, customSubject: '' })}
              >
                {s}
              </button>
            ))}
          </div>

          <input
            className="chat-input"
            placeholder="Or type your own subject..."
            value={data.customSubject}
            onChange={(e) => setData({ ...data, customSubject: e.target.value, subject: '' })}
          />

          <div className="express-wizard__grade">
            <div className="section-label">GRADE LEVEL</div>
            <div className="medium-grid">
              {GRADES.map((g) => (
                <button
                  key={g}
                  className={`medium-btn ${data.grade === g ? 'medium-btn--active' : ''}`}
                  onClick={() => setData({ ...data, grade: g })}
                >
                  {g}
                </button>
              ))}
            </div>
          </div>

          <button
            className="chat-send begin-btn"
            disabled={!subject}
            onClick={() => setStep(2)}
          >
            NEXT →
          </button>
        </div>
      </div>
    );
  }

  // ── Step 2: What should students learn? ──
  if (step === 2) {
    return (
      <div className="express-wizard">
        <div className="express-wizard__header">
          <div className="express-wizard__icon">⚡</div>
          <div className="express-wizard__title">EXPRESS MODE</div>
          <div className="express-wizard__subtitle">
            Teaching: <strong>{subject}</strong> ({data.grade})
          </div>
        </div>

        <div className="express-wizard__step">
          <div className="express-wizard__step-label">STEP 2 OF 3</div>
          <div className="express-wizard__question">
            What should students be able to DO after this lesson?
          </div>

          <textarea
            className="express-wizard__textarea"
            placeholder="e.g. Students can identify the 5 components of fitness and explain why each matters for their daily life..."
            value={data.goal}
            onChange={(e) => setData({ ...data, goal: e.target.value })}
            rows={4}
          />

          <input
            className="chat-input"
            placeholder="Who are your students? (optional — e.g. 'shy 9th graders, mixed abilities')"
            value={data.audience}
            onChange={(e) => setData({ ...data, audience: e.target.value })}
          />

          <div className="express-wizard__nav">
            <button className="nav-link" onClick={() => setStep(1)}>← BACK</button>
            <button
              className="chat-send begin-btn"
              disabled={!data.goal.trim()}
              onClick={() => setStep(3)}
            >
              NEXT →
            </button>
          </div>
        </div>
      </div>
    );
  }

  // ── Step 3: Choose output format ──
  if (step === 3 && !result) {
    return (
      <div className="express-wizard">
        <div className="express-wizard__header">
          <div className="express-wizard__icon">⚡</div>
          <div className="express-wizard__title">EXPRESS MODE</div>
          <div className="express-wizard__subtitle">
            Teaching: <strong>{subject}</strong> ({data.grade})
          </div>
        </div>

        <div className="express-wizard__step">
          <div className="express-wizard__step-label">STEP 3 OF 3</div>
          <div className="express-wizard__question">Choose your output format</div>

          <div className="express-wizard__formats">
            {FORMATS.map((f) => (
              <button
                key={f.value}
                className={`express-format-card ${data.format === f.value ? 'express-format-card--active' : ''}`}
                onClick={() => setData({ ...data, format: f.value })}
              >
                <div className="express-format-card__label">{f.label}</div>
                <div className="express-format-card__desc">{f.desc}</div>
              </button>
            ))}
          </div>

          <div className="express-wizard__nav">
            <button className="nav-link" onClick={() => setStep(2)}>← BACK</button>
            <button
              className="chat-send begin-btn"
              disabled={generating}
              onClick={handleGenerate}
            >
              {generating ? '⚡ GENERATING...' : '⚡ GENERATE'}
            </button>
          </div>
        </div>
      </div>
    );
  }

  // ── Result ──
  if (result) {
    return (
      <div className="express-wizard">
        <div className="express-wizard__header">
          <div className="express-wizard__icon">✅</div>
          <div className="express-wizard__title">READY TO EXPORT</div>
          <div className="express-wizard__subtitle">
            {result.subject} — {result.grade}
          </div>
        </div>

        <div className="express-wizard__result">
          <div className="express-result__card">
            <div className="express-result__title">{result.title}</div>
            <div className="express-result__goal">{result.goal}</div>
            <div className="express-result__format">
              Format: {FORMATS.find(f => f.value === result.format)?.label || result.format}
            </div>
          </div>

          <div className="express-wizard__actions">
            <button
              className="chat-send begin-btn"
              onClick={() => {
                if (onComplete) onComplete(result);
              }}
            >
              📥 EXPORT
            </button>
            <button className="nav-link" onClick={() => { setResult(null); setStep(1); }}>
              ↺ START OVER
            </button>
          </div>
        </div>
      </div>
    );
  }

  return null;
}
