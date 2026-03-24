import React, { useState, useEffect } from 'react';
import CreepCard from './CreepCard';
import PearlCard from './PearlCard';
import TrainStatus from './TrainStatus';

export default function GameHUD({ quest, bestiary, onRefetch }) {
  const [character, setCharacter] = useState(null);

  // Fetch character sheet data (for Session Zero display)
  useEffect(() => {
    const fetchCharacter = async () => {
      try {
        const res = await fetch('/api/character');
        if (res.ok) setCharacter(await res.json());
      } catch { /* silent */ }
    };
    fetchCharacter();
    const interval = setInterval(fetchCharacter, 10000);
    return () => clearInterval(interval);
  }, []);

  // Also refetch when quest XP changes (e.g. after Session Zero completes)
  useEffect(() => {
    const refresh = async () => {
      try {
        const res = await fetch('/api/character');
        if (res.ok) setCharacter(await res.json());
      } catch { /* silent */ }
    };
    refresh();
  }, [quest?.xp]);

  return (
    <div className="game-hud">
      {/* ── Journey ── */}
      <div className="hud-section">
        <div className="hud-section__header">JOURNEY</div>
        <PearlCard pearl={quest?.pearl} onRefetch={onRefetch} />
        <TrainStatus quest={quest} character={character} />
      </div>

      {/* ── Identity ── */}
      <div className="hud-section">
        <div className="hud-section__header">IDENTITY</div>
        <div className="card">
          <div className="card-header">YARDMASTER</div>
          <div className="card-subtitle">
            {quest?.subject ? `SME: ${quest.subject}` : 'No subject'}
          </div>
          <div className="hud-stat-row">
            <span className="stat-xp">⚡ XP {quest?.xp || 0}</span>
            <span className="stat-coal">🪨 Coal {quest?.coal || 0}</span>
          </div>

          {/* Session Zero — Character Creation Answers */}
          {(character?.experience || character?.audience || character?.success_vision) && (
            <div className="session-zero-info">
              {character.experience && (
                <div className="session-zero-info__item">
                  <span className="session-zero-info__icon session-zero-info__icon--gold">📋</span>
                  {character.experience}
                </div>
              )}
              {character.audience && (
                <div className="session-zero-info__item">
                  <span className="session-zero-info__icon session-zero-info__icon--blue">👥</span>
                  {character.audience}
                </div>
              )}
              {character.success_vision && (
                <div className="session-zero-info__item">
                  <span className="session-zero-info__icon session-zero-info__icon--green">🎯</span>
                  {character.success_vision}
                </div>
              )}
            </div>
          )}

          {quest?.party?.length > 0 && (
            <div className="hud-party">
              {quest.party.map((p, i) => (
                <span
                  key={p.id || i}
                  className={`hud-party__member ${p.active ? 'hud-party__member--active' : ''}`}
                  title={p.role || ''}
                >
                  {p.avatar || '👤'} {p.name || p.id}
                </span>
              ))}
            </div>
          )}
        </div>
      </div>

      {/* ── Collection ── */}
      <div className="hud-section">
        <div className="hud-section__header">COLLECTION</div>

        {/* Inventory */}
        {quest?.inventory?.length > 0 && (
          <div className="card">
            <div className="card-header">INVENTORY</div>
            {quest.inventory.map((item, i) => (
              <div key={i} className="session-zero-info__item">
                {item}
              </div>
            ))}
          </div>
        )}

        {/* Bestiary */}
        <div className="card">
          <div className="card-header">BESTIARY</div>
          <div className="card-subtitle">
            {bestiary
              ? `${bestiary.tamed || 0} tamed / ${bestiary.total || 0} total`
              : 'No creeps discovered'}
          </div>
          <div className="bestiary-list">
            {bestiary?.creeps?.slice(0, 8).map((c, i) => (
              <CreepCard key={i} creep={c} />
            ))}
            {!bestiary?.creeps?.length && (
              <div className="bestiary-empty">
                Chat with Pete to discover vocabulary creeps
              </div>
            )}
          </div>
        </div>

        {/* Sacred Circuitry */}
        <div className="card">
          <div className="card-header">CIRCUITRY</div>
          <div className="card-subtitle">ADDIECRAPEYE SPIRAL</div>
          <div className="circuitry-grid">
            {[
              { label: 'Scope', icon: '⊕', color: 'var(--gold)' },
              { label: 'Build', icon: '⧉', color: 'var(--blue)' },
              { label: 'Listen', icon: '⌬', color: 'var(--purple)' },
              { label: 'Ship', icon: '◈', color: 'var(--green)' },
            ].map((q) => (
              <div key={q.label} className="circuitry-cell" style={{ color: q.color }}>
                <div className="circuitry-cell__icon">{q.icon}</div>
                <div className="circuitry-cell__label">{q.label}</div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
