import React, { useState, useEffect } from 'react';
import CreepCard from './CreepCard';
import PearlCard from './PearlCard';
import TrainStatus from './TrainStatus';

export default function GameHUD({ quest, bestiary, onRefetch, sseEvents }) {
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

  // Sync instantaneous character sheet updates from Iron Road chat stream
  useEffect(() => {
    const handleSse = (data) => {
      if (data?.type === 'character_update') {
        setCharacter(prev => ({ ...prev, ...data }));
      }
    };

    if (sseEvents?.length) {
      sseEvents.forEach(ev => {
        try { handleSse(typeof ev === 'string' ? JSON.parse(ev) : ev); } catch {}
      });
    }

    const streamListener = (e) => handleSse(e.detail);
    window.addEventListener('trinity-stream-event', streamListener);
    return () => window.removeEventListener('trinity-stream-event', streamListener);
  }, [sseEvents]);

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
          <div className="hud-safety-badges">
            <span className="safety-badge safety-badge--active" title="CowCatcher AI Content Filter — Active">🛡️ CowCatcher</span>
            <span className="safety-badge safety-badge--active" title="EdgeGuard Security Middleware — Active">🔒 EdgeGuard</span>
            <span className="safety-badge safety-badge--active" title="Demo Mode — Visitor restrictions active">🎓 Demo</span>
          </div>

          {/* Session Zero — Character Creation Answers */}
          {(character?.experience || character?.audience || character?.success_vision) && (
            <div className="session-zero-info">
              <button
                onClick={async () => {
                  await fetch('/api/character', {
                    method: 'POST', headers: {'Content-Type':'application/json'},
                    body: JSON.stringify({ experience: '', audience: '', success_vision: '' })
                  });
                  setCharacter(c => ({ ...c, experience: null, audience: null, success_vision: null }));
                }}
                style={{ float: 'right', background: 'none', border: '1px solid rgba(207,185,145,0.15)', borderRadius: '4px', color: '#6B7280', cursor: 'pointer', padding: '1px 6px', fontSize: '8px' }}
                title="Clear Session Zero data"
              >✕</button>
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
        </div>
      </div>

      {/* ── Collection (Progressive Disclosure — only show when populated) ── */}
      <div className="hud-section">
        <div className="hud-section__header">COLLECTION</div>

        {/* Inventory — only when items exist */}
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

        {/* Bestiary — only when creeps have been discovered */}
        {bestiary?.creeps?.length > 0 && (
          <div className="card">
            <div className="card-header">BESTIARY</div>
            <div className="card-subtitle">
              {bestiary.tamed || 0} tamed / {bestiary.total || 0} total
            </div>
            <div className="bestiary-list">
              {bestiary.creeps.slice(0, 8).map((c, i) => (
                <CreepCard key={i} creep={c} />
              ))}
            </div>
          </div>
        )}

        {/* VAAM Vocabulary Activity Feed */}
        <VaamActivityFeed sseEvents={sseEvents} />

        {/* Sacred Circuitry — wired to live backend data */}
        <CircuitryCard />

        {/* Book of the Bible — only when content exists */}
        <BookSection />
      </div>
    </div>
  );
}

// ── VAAM Vocabulary Activity Feed (P1) ──
function VaamActivityFeed({ sseEvents }) {
  const [feed, setFeed] = React.useState([]);

  React.useEffect(() => {
    const handleVaam = (data) => {
      if (data?.type === 'vaam' && data.detections?.length > 0) {
        setFeed(prev => {
          const newEntries = data.detections.map(d => ({
            word: d.word,
            coal: d.coal_yield || 0,
            mastered: d.mastered,
            id: Date.now() + Math.random()
          }));
          return [...newEntries, ...prev].slice(0, 10);
        });
      }
    };

    if (sseEvents?.length) {
      sseEvents.forEach(ev => {
        try { handleVaam(typeof ev === 'string' ? JSON.parse(ev) : ev); } catch {}
      });
    }

    const streamListener = (e) => handleVaam(e.detail);
    window.addEventListener('trinity-stream-event', streamListener);
    return () => window.removeEventListener('trinity-stream-event', streamListener);
  }, [sseEvents]);

  if (feed.length === 0) return null;

  return (
    <div className="card">
      <div className="card-header">VAAM ACTIVITY</div>
      <div className="card-subtitle">Recent vocabulary matrix catches</div>
      <div className="vaam-feed" style={{ display: 'flex', flexDirection: 'column', gap: '4px', fontSize: '11px' }}>
        {feed.map(f => (
          <div key={f.id} style={{ display: 'flex', justifyContent: 'space-between', padding: '4px', background: 'rgba(0,0,0,0.2)', borderLeft: `2px solid ${f.mastered ? 'var(--gold)' : 'var(--blue)'}` }}>
            <span style={{ color: f.mastered ? 'var(--gold)' : 'var(--text)' }}>
              {f.mastered ? '★ ' : ''}{f.word}
            </span>
            <span style={{ color: 'var(--green)' }}>+{f.coal} 🪨</span>
          </div>
        ))}
      </div>
    </div>
  );
}

// ── Sacred Circuitry — Live VAAM State (C3) ──
function CircuitryCard() {
  const [circuitry, setCircuitry] = React.useState(null);

  React.useEffect(() => {
    fetch('/api/quest/circuitry')
      .then(r => r.json())
      .then(setCircuitry)
      .catch(() => {});
    // Re-poll every 30s to catch VAAM updates
    const interval = setInterval(() => {
      fetch('/api/quest/circuitry')
        .then(r => r.json())
        .then(setCircuitry)
        .catch(() => {});
    }, 30000);
    return () => clearInterval(interval);
  }, []);

  const QUADRANTS = [
    { key: 'scope', label: 'Scope', icon: '⊕', color: 'var(--gold)' },
    { key: 'build', label: 'Build', icon: '⧉', color: 'var(--blue)' },
    { key: 'listen', label: 'Listen', icon: '⌬', color: 'var(--purple)' },
    { key: 'ship', label: 'Ship', icon: '◈', color: 'var(--green)' },
  ];

  return (
    <div className="card">
      <div className="card-header">CIRCUITRY</div>
      <div className="card-subtitle">ADDIECRAPEYE SPIRAL</div>
      <div className="circuitry-grid">
        {QUADRANTS.map((q) => {
          const data = circuitry?.quadrants?.[q.key];
          const activation = data?.activation ?? 0;
          const words = data?.word_count ?? 0;
          return (
            <div key={q.key} className="circuitry-cell" style={{ color: q.color }}>
              <div className="circuitry-cell__icon">{q.icon}</div>
              <div className="circuitry-cell__label">{q.label}</div>
              {words > 0 && (
                <div className="circuitry-cell__count">{words}</div>
              )}
              {activation > 0 && (
                <div className="circuitry-cell__bar">
                  <div
                    className="circuitry-cell__fill"
                    style={{ width: `${Math.min(activation * 100, 100)}%`, background: q.color }}
                  />
                </div>
              )}
            </div>
          );
        })}
      </div>
      {circuitry?.total_words > 0 && (
        <div className="card-subtitle" style={{ marginTop: '4px' }}>
          {circuitry.total_words} words in matrix
        </div>
      )}
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

  const isEmpty = !book?.chapters?.length && !book?.narrative;
  if (isEmpty) return null; // Progressive disclosure

  return (
    <div className="card">
      <div className="card-header">📖 THE BOOK</div>
      <div className="card-subtitle">
        {book?.chapters?.length || 0} chapters written
        <button
          className="book-write-btn"
          onClick={generateNarrative}
          disabled={generating}
        >
          {generating ? '✍️…' : '✍️ Write'}
        </button>
      </div>
      {book?.narrative && (
        <div className="book-narrative">
          {book.narrative}
        </div>
      )}
      {book?.chapters?.slice(-3).map((ch, i) => (
        <div key={i} className="book-chapter">
          📜 {ch.title || ch.summary || `Chapter ${i+1}`}
        </div>
      ))}
    </div>
  );
}
