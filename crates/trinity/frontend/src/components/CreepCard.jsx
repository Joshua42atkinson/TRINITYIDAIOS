import React from 'react';

export default function CreepCard({ creep }) {
  if (!creep) return null;

  const pct = Math.round((creep.taming_progress || 0) * 100);
  const grade = pct >= 70 ? 'high' : pct >= 40 ? 'mid' : 'low';
  const status = creep.tamed ? 'tamed' : (creep.tameable ? 'tameable' : 'wild');

  return (
    <div className="creep-card">
      <div className="creep-card__row">
        <span className="creep-word">{creep.word}</span>
        <span className={`creep-tag ${status}`}>{status}</span>
      </div>
      <div className="creep-meta">
        <span className="creep-power">⚡{creep.power || 0}</span>
        <span>×{creep.encounters || 0}</span>
      </div>
      <div className="taming-bar">
        <div className={`taming-fill ${grade}`} style={{ width: `${pct}%` }} />
      </div>
    </div>
  );
}
