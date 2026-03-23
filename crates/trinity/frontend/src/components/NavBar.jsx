import React from 'react';

export default function NavBar({ quest, activeTab, onTabChange, onNewJourney }) {
  const tabs = [
    { id: 'ironroad', label: 'Iron Road' },
    { id: 'art',      label: 'ART Studio' },
    { id: 'yard',     label: 'Yardmaster' },
    { id: 'scorecard', label: 'Scorecard' },
    { id: 'voice',    label: 'Voice' },
  ];

  return (
    <nav className="nav">
      <div className="nav-brand">TRINITY</div>
      <div className="nav-links">
        {tabs.map((t) => (
          <button
            key={t.id}
            id={`nav-${t.id}`}
            className={`nav-link ${activeTab === t.id ? 'active' : ''}`}
            onClick={() => onTabChange(t.id)}
          >
            {t.label}
          </button>
        ))}
      </div>
      <div className="nav-status">
        <span className={`status-dot ${quest?.phase ? 'connected' : ''}`}></span>
        <span>{quest?.phase || 'awaiting…'}</span>
        <span className="sep">|</span>
        <span>Ch {quest?.chapter || '—'}</span>
        {quest?.subject && onNewJourney && (
          <>
            <span className="sep">|</span>
            <button
              id="nav-new-journey-btn"
              className="nav-new-journey"
              onClick={onNewJourney}
            >
              ↺ NEW JOURNEY
            </button>
          </>
        )}
      </div>
    </nav>
  );
}
