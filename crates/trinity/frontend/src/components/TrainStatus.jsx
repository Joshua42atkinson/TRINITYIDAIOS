import React from 'react';

export default function TrainStatus({ quest }) {
  const coal = quest?.coal ?? 100;
  const steam = quest?.steam ?? 0;
  const xp = quest?.xp ?? 0;
  const resonance = quest?.resonance ?? 1;

  const coalColor = coal > 50 ? 'var(--green)' : coal > 20 ? 'var(--gold)' : 'var(--red)';
  const steamColor = 'var(--blue)';
  const velocity = Math.min((steam / 10), 10).toFixed(1);

  return (
    <div className="card" id="train-status">
      <div className="card-header train-header">
        <span>🚂</span>
        <span>LOCOMOTIVE</span>
        <span className="train-velocity">v{velocity}</span>
      </div>

      <div className="train-gauge">
        <div className="train-gauge__label">
          <span className="train-gauge__name">🪨 COAL</span>
          <span className="train-gauge__value" style={{ color: coalColor }}>
            {coal.toFixed(0)}%
          </span>
        </div>
        <div className="progress-bar">
          <div className="progress-fill" style={{ width: `${coal}%`, background: coalColor }} />
        </div>
      </div>

      <div className="train-gauge">
        <div className="train-gauge__label">
          <span className="train-gauge__name">💨 STEAM</span>
          <span className="train-gauge__value" style={{ color: steamColor }}>
            {steam.toFixed(0)}
          </span>
        </div>
        <div className="progress-bar">
          <div className="progress-fill" style={{ width: `${Math.min(steam, 100)}%`, background: steamColor }} />
        </div>
      </div>

      <div className="train-stats">
        <span className="train-stat">
          ✨ Resonance: <span className="train-stat__value">{resonance}</span>
        </span>
        <span className="train-stat">
          ⚡ XP: <span className="train-stat__value">{xp}</span>
        </span>
      </div>
    </div>
  );
}
