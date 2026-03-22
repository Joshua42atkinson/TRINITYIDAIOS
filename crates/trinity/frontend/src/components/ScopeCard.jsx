import React from 'react';

export default function ScopeCard({ word, power, progress, onHope, onNope }) {
  const pct = Math.round((progress || 0) * 100);
  const grade = pct >= 70 ? 'high' : pct >= 40 ? 'mid' : 'low';

  return (
    <div className="scope-card">
      <div className="scope-word">🌟 {word?.toUpperCase()}</div>
      <div className="scope-meta">
        <span>PWR {power || 0}</span>
        <span>Taming: {pct}%</span>
        <div className="taming-bar" style={{ flex: 1 }}>
          <div
            className={`taming-fill ${grade}`}
            style={{ width: `${pct}%` }}
          />
        </div>
      </div>
      <div className="scope-btns">
        <button className="scope-btn hope" onClick={() => onHope?.(word)}>
          ✓ HOPE
        </button>
        <button className="scope-btn nope" onClick={() => onNope?.(word)}>
          ✗ NOPE
        </button>
      </div>
    </div>
  );
}
