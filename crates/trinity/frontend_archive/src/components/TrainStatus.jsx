import React, { useState, useEffect } from 'react';

export default function TrainStatus({ quest, character }) {
  const coal = quest?.coal ?? 100;
  const steam = quest?.steam ?? 0;
  const xp = quest?.xp ?? 0;
  const resonance = quest?.resonance ?? 1;

  // Physics engine metrics from CharacterSheet
  const friction = character?.track_friction ?? 0;
  const vulnerability = character?.vulnerability ?? 0.5;
  const shadow = character?.shadow_status ?? 'Clear';
  const negatives = character?.consecutive_negatives ?? 0;

  // Product Maturity from quest API
  const maturity = quest?.product_maturity;

  // Progressive disclosure: auto-expand advanced gauges when non-default
  const hasAdvancedData = friction > 0 || vulnerability !== 0.5 || shadow !== 'Clear';
  const [showAdvanced, setShowAdvanced] = useState(hasAdvancedData);

  // Auto-expand if values change to non-default
  useEffect(() => {
    if (hasAdvancedData && !showAdvanced) setShowAdvanced(true);
  }, [hasAdvancedData]);

  const coalColor = coal > 50 ? 'var(--green)' : coal > 20 ? 'var(--gold)' : 'var(--red)';
  const steamColor = 'var(--blue)';
  const frictionColor = friction > 50 ? 'var(--red)' : friction > 25 ? 'var(--gold)' : 'var(--green)';
  const vulnColor = vulnerability > 0.7 ? 'var(--red)' : vulnerability > 0.4 ? 'var(--gold)' : 'var(--green)';
  const velocity = Math.min((steam / 10), 10).toFixed(1);

  const shadowIcon = {
    'Clear': '🌕',
    'Stirring': '🌘',
    'Active': '🌑',
    'Processed': '🌕✨',
  }[shadow] || '🌕';

  const shadowColor = {
    'Clear': 'var(--green)',
    'Stirring': 'var(--gold)',
    'Active': 'var(--red)',
    'Processed': 'var(--purple)',
  }[shadow] || 'var(--text-dim)';

  return (
    <div className="card" id="train-status">
      <div className="card-header train-header">
        <span>🚂</span>
        <span>LOCOMOTIVE</span>
        <span className="train-velocity">v{velocity}</span>
      </div>

      {/* Primary Gauges — Always Visible */}
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

      {/* Core Stats Row */}
      <div className="train-stats">
        <span className="train-stat">
          ⚡ XP: <span className="train-stat__value">{xp}</span>
        </span>
        <span className="train-stat">
          ✨ Resonance: <span className="train-stat__value">{resonance}</span>
        </span>
      </div>

      {/* Product Maturity Bar */}
      {maturity && (
        <div className="train-gauge" style={{ marginTop: '8px', borderTop: '1px solid var(--border)', paddingTop: '8px' }}>
          <div className="train-gauge__label">
            <span className="train-gauge__name">📦 PRODUCT</span>
            <span className="train-gauge__value" style={{ color: 'var(--purple)' }}>
              {maturity.score}% — {maturity.label}
            </span>
          </div>
          <div className="progress-bar">
            <div className="progress-fill" style={{ width: `${maturity.score}%`, background: 'var(--purple)' }} />
          </div>
        </div>
      )}

      {/* Advanced Gauges — Progressive Disclosure */}
      <button
        className="train-advanced-toggle"
        onClick={() => setShowAdvanced(s => !s)}
      >
        {showAdvanced ? '▾' : '▸'} ENGINE DIAGNOSTICS
        {hasAdvancedData && <span className="train-advanced-dot" />}
      </button>

      {showAdvanced && (
        <div className="train-advanced">
          <div className="train-gauge">
            <div className="train-gauge__label">
              <span className="train-gauge__name">🔧 FRICTION</span>
              <span className="train-gauge__value" style={{ color: frictionColor }}>
                {friction.toFixed(0)}%
              </span>
            </div>
            <div className="progress-bar">
              <div className="progress-fill" style={{ width: `${friction}%`, background: frictionColor }} />
            </div>
          </div>

          <div className="train-gauge">
            <div className="train-gauge__label">
              <span className="train-gauge__name">🛡️ VULN</span>
              <span className="train-gauge__value" style={{ color: vulnColor }}>
                {(vulnerability * 100).toFixed(0)}%
              </span>
            </div>
            <div className="progress-bar">
              <div className="progress-fill" style={{ width: `${vulnerability * 100}%`, background: vulnColor }} />
            </div>
          </div>

          <div className="train-stats">
            <span className="train-stat">
              {shadowIcon} Shadow: <span className="train-stat__value" style={{ color: shadowColor }}>{shadow}</span>
              {negatives > 0 && <span style={{ color: 'var(--red)', fontSize: '0.75rem' }}> ({negatives}×)</span>}
            </span>
          </div>
        </div>
      )}
    </div>
  );
}

