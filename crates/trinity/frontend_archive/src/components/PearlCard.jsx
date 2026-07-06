import React, { useState } from 'react';

const PHASE_COLORS = {
  Extracting: 'var(--gold)',
  Placing: 'var(--blue)',
  Refining: 'var(--purple)',
  Polished: 'var(--green)',
};

const MEDIUMS = [
  { value: 'Game', icon: '🎮' },
  { value: 'Storyboard', icon: '🎬' },
  { value: 'Simulation', icon: '🔬' },
  { value: 'LessonPlan', icon: '📋' },
  { value: 'Assessment', icon: '📝' },
  { value: 'Book', icon: '📖' },
];

function AlignmentBar({ label, score, color }) {
  const pct = Math.round((score || 0) * 100);
  return (
    <div className="train-gauge">
      <div className="train-gauge__label">
        <span className="train-gauge__name">{label}</span>
        <span className="train-gauge__value" style={{ color: pct >= 60 ? 'var(--green)' : 'var(--text-dim)' }}>
          {pct}%
        </span>
      </div>
      <div className="progress-bar">
        <div className="progress-fill" style={{ width: `${pct}%`, background: color }} />
      </div>
    </div>
  );
}

export default function PearlCard({ pearl, onRefetch }) {
  const [refining, setRefining] = useState(false);
  const [visionInput, setVisionInput] = useState('');
  const [mediumInput, setMediumInput] = useState('');
  const [saving, setSaving] = useState(false);
  const [savedFlash, setSavedFlash] = useState(false);

  if (!pearl) return null;

  const phaseColor = PHASE_COLORS[pearl.phase] || 'var(--gold)';
  const alignPct = Math.round((pearl.alignment || 0) * 100);

  const openRefine = () => {
    setVisionInput(pearl.vision || '');
    setMediumInput(pearl.medium || 'Game');
    setRefining(true);
  };

  const saveRefine = async () => {
    setSaving(true);
    try {
      await fetch('/api/pearl/refine', {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          vision: visionInput.trim() || null,
          medium: mediumInput || null,
        }),
      });
      setSavedFlash(true);
      setTimeout(() => setSavedFlash(false), 1500);
      setRefining(false);
      if (onRefetch) onRefetch();
    } catch (e) {
      console.error('Refine failed:', e);
    }
    setSaving(false);
  };

  return (
    <div className="card" id="pearl-card" style={{
      border: savedFlash ? '1px solid var(--green)' : undefined,
      transition: 'border-color 0.4s ease',
    }}>
      <div className="pearl-header">
        <div className="pearl-header__left">
          <span className="card-header" style={{ margin: 0 }}>PEARL</span>
          {pearl.grade && (
            <span className={`pearl-grade ${alignPct >= 60 ? 'pearl-grade--good' : 'pearl-grade--poor'}`}>
              {pearl.grade}
            </span>
          )}
        </div>
        <button
          id="pearl-refine-btn"
          className={`pearl-refine-btn ${refining ? 'pearl-refine-btn--active' : ''}`}
          onClick={refining ? () => setRefining(false) : openRefine}
        >
          {refining ? '✕ CLOSE' : '✏ REFINE'}
        </button>
      </div>

      {refining && (
        <div className="pearl-refine-form">
          <div className="pearl-refine-label">✏ REFINE YOUR PEARL</div>
          <div>
            <div className="section-label">VISION</div>
            <input
              id="pearl-vision-input"
              className="vision-input"
              placeholder="What should the output feel like?"
              value={visionInput}
              onChange={e => setVisionInput(e.target.value)}
            />
          </div>
          <div>
            <div className="section-label">MEDIUM</div>
            <div className="medium-grid">
              {MEDIUMS.map(m => (
                <button
                  key={m.value}
                  className={`medium-btn ${mediumInput === m.value ? 'medium-btn--active' : ''}`}
                  onClick={() => setMediumInput(m.value)}
                >
                  {m.icon} {m.value}
                </button>
              ))}
            </div>
          </div>
          <button
            id="pearl-save-btn"
            className="pearl-save-btn"
            onClick={saveRefine}
            disabled={saving}
          >
            {saving ? 'SAVING...' : '✓ SAVE REFINEMENT'}
          </button>
        </div>
      )}

      <div className="pearl-subject">
        {pearl.medium_icon} {pearl.subject}
        <span className="pearl-subject__medium">via {pearl.medium}</span>
      </div>

      {pearl.has_vision && pearl.vision ? (
        <div className="pearl-vision">"{pearl.vision}"</div>
      ) : (
        <div className="pearl-vision--empty">⚠ No vision set — click Refine to add one</div>
      )}

      <div className="pearl-phase">
        <span style={{ fontSize: '14px' }}>{pearl.phase_icon}</span>
        <span className="pearl-phase__badge" style={{ borderColor: phaseColor, color: phaseColor }}>
          {pearl.phase}
        </span>
        {pearl.refined_count > 0 && (
          <span className="pearl-alignment__label">×{pearl.refined_count}</span>
        )}
      </div>

      <AlignmentBar label="ADDIE" score={pearl.addie_score} color="var(--gold)" />
      <AlignmentBar label="CRAP" score={pearl.crap_score} color="var(--blue)" />
      <AlignmentBar label="EYE" score={pearl.eye_score} color="var(--purple)" />

      <div className="pearl-alignment">
        <span className="pearl-alignment__label">ALIGNMENT</span>
        <span className="pearl-alignment__value" style={{
          color: alignPct >= 60 ? 'var(--green)' : alignPct >= 30 ? 'var(--gold)' : 'var(--red)',
        }}>
          {alignPct}%
        </span>
      </div>

      {savedFlash && <div className="pearl-saved">✓ PEARL REFINED</div>}
    </div>
  );
}
