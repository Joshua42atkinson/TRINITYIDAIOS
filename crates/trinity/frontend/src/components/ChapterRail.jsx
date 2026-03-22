import React from 'react';

// Hero's Journey chapter titles — one per ADDIECRAPEYE station
const CHAPTER_TITLES = [
  'The Ordinary World',
  'The Call to Adventure',
  'Refusal of the Call',
  'Meeting the Mentor',
  'Crossing the Threshold',
  'Tests, Allies & Enemies',
  'Approach to the Cave',
  'The Ordeal',
  'The Reward',
  'The Road Back',
  'The Resurrection',
  'Return with Elixir',
];

// Quadrant group labels — Bible language: Extract → Place → Refine
const GROUPS = [
  { label: 'EXTRACT — ADDIE', sublabel: 'Analyze · Design · Build · Deploy · Evaluate', range: [0, 5] },
  { label: 'PLACE — CRAP',    sublabel: 'Contrast · Repeat · Align · Proximity',         range: [5, 9] },
  { label: 'REFINE — EYE',    sublabel: 'Envision · Yoke · Evolve',                       range: [9, 12] },
];

export default function ChapterRail({ phases, currentPhaseIndex, completedPhases, onPhaseClick }) {
  return (
    <div className="chapter-rail">
      <div className="card-header card-header--rail">
        THE IRON ROAD
      </div>

      {GROUPS.map((g, gi) => (
        <React.Fragment key={gi}>
          {/* Group header — ornamental divider between groups */}
          <div className={`rail-group ${gi > 0 ? '' : ''}`}>
            <div className="rail-group__label">
              {g.label}
            </div>
            <div className="rail-group__sublabel">
              {g.sublabel}
            </div>
          </div>

          {gi > 0 && <div className="book-divider--light book-divider" />}

          {phases.slice(g.range[0], g.range[1]).map((phase, i) => {
            const idx = g.range[0] + i;
            const isActive = idx === currentPhaseIndex;
            const isDone = completedPhases?.includes(phase.name);
            const chapterTitle = CHAPTER_TITLES[idx];

            return (
              <div
                key={idx}
                id={`chapter-${idx}`}
                className={`chapter-item ${isActive ? 'active' : ''} ${isDone ? 'done' : ''}`}
                onClick={() => onPhaseClick(idx, phase.name)}
                title={`${phase.name} — ${chapterTitle}\nBloom's: ${phase.blooms} · Circuit: ${phase.circuit}`}
              >
                <span className="ch-num">{idx + 1}</span>
                <span className="ch-icon">{phase.icon}</span>
                <div className="ch-name-wrap">
                  <span className="ch-name">{phase.name}</span>
                  {isActive && (
                    <div className="ch-subtitle">
                      {chapterTitle}
                    </div>
                  )}
                </div>
                <span className="ch-blooms">{phase.blooms}</span>
              </div>
            );
          })}
        </React.Fragment>
      ))}

      {/* Progress summary */}
      <div className="rail-progress">
        <div className="card-subtitle">PROGRESS</div>
        <div className="progress-bar rail-progress__bar">
          <div
            className="progress-fill"
            style={{
              width: `${((currentPhaseIndex + 1) / 12) * 100}%`,
              background: currentPhaseIndex < 5
                ? 'var(--group-addie)'
                : currentPhaseIndex < 9
                  ? 'var(--group-crap)'
                  : 'var(--group-eye)',
              transition: 'width 0.8s ease, background 0.5s ease',
            }}
          />
        </div>
        <div className="rail-progress__info">
          <span className="rail-progress__chapter">
            {CHAPTER_TITLES[currentPhaseIndex]?.split(' ').slice(0, 2).join(' ')}
          </span>
          <span>{currentPhaseIndex + 1}/12</span>
        </div>
      </div>
    </div>
  );
}
