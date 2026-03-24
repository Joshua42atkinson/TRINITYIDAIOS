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
            <span className="stat-xp">⚡ XP {quest?.xp_earned || 0}</span>
            <span className="stat-coal">🪨 Coal {Math.round(quest?.coal_used || 0)}</span>
            <span className="stat-steam">💨 Steam {Math.round(quest?.steam_generated || 0)}</span>
          </div>
          <div className="hud-safety-badges">
            <span className="safety-badge safety-badge--active" title="CowCatcher AI Content Filter — Active">🛡️ CowCatcher</span>
            <span className="safety-badge safety-badge--active" title="EdgeGuard Security Middleware — Active">🔒 EdgeGuard</span>
            <span className="safety-badge safety-badge--active" title="Demo Mode — Visitor restrictions active">🎓 Demo</span>
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
        {/* Party Members */}
        <div className="hud-party">
          {(quest?.party || []).map((m) => (
            <button
              key={m.id}
              className={`hud-party__member ${m.active ? 'hud-party__member--active' : ''}`}
              onClick={() => {
                fetch('/api/quest/party', {
                  method: 'POST', headers: { 'Content-Type': 'application/json' },
                  body: JSON.stringify({ member_id: m.id }),
                }).catch(() => {});
              }}
              title={m.name || m.id}
            >
              {m.id === 'pete' ? '🎓' : m.id === 'art' ? '🎨' : '🔧'} {m.id}
            </button>
          ))}
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
        {/* Book of the Bible — Narrative Chapters */}
        <BookSection />
      </div>
    </div>
  );
}

function BookSection() {
  const [book, setBook] = React.useState(null);
  const [generating, setGenerating] = React.useState(false);

  React.useEffect(() => {
    fetch('/api/book').then(r => r.json()).then(setBook).catch(() => {});
  }, []);

  const generateNarrative = async () => {
    setGenerating(true);
    try {
      const res = await fetch('/api/narrative/generate');
      if (res.ok) {
        const data = await res.json();
        setBook(prev => prev ? { ...prev, narrative: data.narrative || data.text } : data);
      }
    } catch {}
    setGenerating(false);
  };

  return (
    <div className="card">
      <div className="card-header">📖 THE BOOK</div>
      <div className="card-subtitle">
        {book?.chapters?.length || 0} chapters written
        <button
          onClick={generateNarrative}
          disabled={generating}
          style={{
            float: 'right', background: 'none', border: '1px solid rgba(207,185,145,0.2)',
            borderRadius: '4px', color: '#CFB991', cursor: 'pointer', padding: '1px 6px', fontSize: '9px',
          }}
        >
          {generating ? '✍️…' : '✍️ Write'}
        </button>
      </div>
      {book?.narrative && (
        <div style={{ fontSize: '11px', color: '#94a3b8', fontStyle: 'italic', fontFamily: "'Crimson Text', serif", padding: '6px 0', maxHeight: '120px', overflow: 'auto' }}>
          {book.narrative}
        </div>
      )}
      {book?.chapters?.slice(-3).map((ch, i) => (
        <div key={i} style={{ fontSize: '10px', color: '#6B7280', borderTop: '1px solid rgba(207,185,145,0.06)', padding: '4px 0' }}>
          📜 {ch.title || ch.summary || `Chapter ${i+1}`}
        </div>
      ))}
      {!book?.chapters?.length && !book?.narrative && (
        <div style={{ fontSize: '10px', color: '#4B5563', fontStyle: 'italic', padding: '8px 0' }}>
          No story yet. Complete phases to generate narrative chapters.
        </div>
      )}
    </div>
  );
}
